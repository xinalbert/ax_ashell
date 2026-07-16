//! Telnet backend with conservative RFC 854 option negotiation.

use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use anyhow::{Context, Result};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc,
    task::JoinHandle,
};

use crate::{
    app::RuntimeTaskTracker,
    events::{BackendEvent, BackendEventSender},
    session::Session,
    terminal::{BackendCommand, BackendShutdown, BackendTx},
};

const IAC: u8 = 255;
const DONT: u8 = 254;
const DO: u8 = 253;
const WONT: u8 = 252;
const WILL: u8 = 251;
const SB: u8 = 250;
const SE: u8 = 240;
const OPT_ECHO: u8 = 1;
const OPT_SGA: u8 = 3;
const OPT_NAWS: u8 = 31;
const TELNET_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(2);

struct TelnetBackendShutdown {
    commands: mpsc::UnboundedSender<BackendCommand>,
    join: Mutex<Option<JoinHandle<()>>>,
    runtime: tokio::runtime::Handle,
    task_tracker: RuntimeTaskTracker,
    shutdown_requested: Arc<AtomicBool>,
}

impl BackendShutdown for TelnetBackendShutdown {
    fn shutdown(&self) {
        if self.shutdown_requested.swap(true, Ordering::SeqCst) {
            return;
        }
        let _ = self.commands.send(BackendCommand::Close);
        let Some(mut join) = self
            .join
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take()
        else {
            return;
        };
        let task_lease = self.task_tracker.acquire();
        self.runtime.spawn(async move {
            let _task_lease = task_lease;
            if tokio::time::timeout(TELNET_SHUTDOWN_TIMEOUT, &mut join)
                .await
                .is_err()
            {
                join.abort();
                let _ = join.await;
            }
        });
    }
}

pub(crate) fn spawn_telnet_terminal(
    runtime: &tokio::runtime::Handle,
    task_tracker: RuntimeTaskTracker,
    tab_id: String,
    session: Session,
    cols: u16,
    rows: u16,
    events: BackendEventSender,
) -> BackendTx {
    let (commands, receiver) = mpsc::unbounded_channel();
    let shutdown_requested = Arc::new(AtomicBool::new(false));
    let task_shutdown_requested = shutdown_requested.clone();
    let task_tab_id = tab_id.clone();
    let task_lease = task_tracker.acquire();
    let join = runtime.spawn(async move {
        let _task_lease = task_lease;
        if let Err(err) = run_telnet(
            task_tab_id.clone(),
            session,
            cols,
            rows,
            receiver,
            events.clone(),
        )
        .await
            && !task_shutdown_requested.load(Ordering::SeqCst)
        {
            let _ = events
                .send(BackendEvent::Closed {
                    tab_id: task_tab_id,
                    reason: format!("telnet error: {err:#}"),
                })
                .await;
        }
    });
    BackendTx::Telnet {
        commands: commands.clone(),
        shutdown: Arc::new(TelnetBackendShutdown {
            commands,
            join: Mutex::new(Some(join)),
            runtime: runtime.clone(),
            task_tracker,
            shutdown_requested,
        }),
    }
}

async fn run_telnet(
    tab_id: String,
    mut session: Session,
    cols: u16,
    rows: u16,
    mut commands: mpsc::UnboundedReceiver<BackendCommand>,
    events: BackendEventSender,
) -> Result<()> {
    if session.host.trim().is_empty() {
        anyhow::bail!("Telnet host is required");
    }
    let port = if session.port == 0 { 23 } else { session.port };
    session.port = port;
    let stream = crate::backend::proxy::connect(&session)
        .await
        .with_context(|| format!("connect Telnet {}:{port}", session.host))?;
    let _ = events
        .send(BackendEvent::Connected {
            tab_id: tab_id.clone(),
        })
        .await;
    let _ = events
        .send(BackendEvent::Status {
            tab_id: tab_id.clone(),
            text: format!("telnet connected: {}:{port}", session.host),
        })
        .await;

    let (mut reader, mut writer) = tokio::io::split(stream);
    let mut hello = vec![IAC, WILL, OPT_SGA, IAC, WILL, OPT_NAWS];
    hello.extend(naws(cols, rows));
    writer
        .write_all(&hello)
        .await
        .context("send Telnet greeting")?;
    writer.flush().await.ok();

    let mut parser = TelnetParser::default();
    let mut buffer = [0u8; 4096];
    loop {
        tokio::select! {
            command = commands.recv() => match command {
                Some(BackendCommand::Input(bytes)) => {
                    writer.write_all(&escape_iac(&bytes)).await.context("write Telnet input")?;
                    writer.flush().await.ok();
                }
                Some(BackendCommand::Resize { cols, rows }) => {
                    writer.write_all(&naws(cols, rows)).await.context("send Telnet resize")?;
                    writer.flush().await.ok();
                }
                Some(BackendCommand::Close) | None => break,
                Some(BackendCommand::SampleMetrics { .. })
                | Some(BackendCommand::CheckConnection { .. })
                | Some(BackendCommand::QueryWorkingDirectory) => {}
            },
            result = reader.read(&mut buffer) => match result {
                Ok(0) => break,
                Ok(read) => {
                    let (data, reply) = parser.process(&buffer[..read]);
                    if !reply.is_empty() {
                        writer.write_all(&reply).await.context("reply to Telnet negotiation")?;
                        writer.flush().await.ok();
                    }
                    if !data.is_empty() {
                        let _ = events.send(BackendEvent::Output {
                            tab_id: tab_id.clone(),
                            bytes: data,
                        }).await;
                    }
                }
                Err(err) => return Err(err).context("read Telnet output"),
            },
        }
    }

    let _ = events
        .send(BackendEvent::Closed {
            tab_id,
            reason: "telnet connection closed".into(),
        })
        .await;
    Ok(())
}

#[derive(Default)]
struct TelnetParser {
    state: TelnetState,
}

#[derive(Default)]
enum TelnetState {
    #[default]
    Data,
    Iac,
    Option(u8),
    Subnegotiation,
    SubnegotiationIac,
}

impl TelnetParser {
    fn process(&mut self, input: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mut data = Vec::with_capacity(input.len());
        let mut reply = Vec::new();
        for &byte in input {
            match self.state {
                TelnetState::Data => {
                    self.state = if byte == IAC {
                        TelnetState::Iac
                    } else {
                        data.push(byte);
                        TelnetState::Data
                    };
                }
                TelnetState::Iac => match byte {
                    IAC => {
                        data.push(IAC);
                        self.state = TelnetState::Data;
                    }
                    DO | DONT | WILL | WONT => self.state = TelnetState::Option(byte),
                    SB => self.state = TelnetState::Subnegotiation,
                    _ => self.state = TelnetState::Data,
                },
                TelnetState::Option(command) => {
                    reply.extend(negotiation_reply(command, byte));
                    self.state = TelnetState::Data;
                }
                TelnetState::Subnegotiation => {
                    if byte == IAC {
                        self.state = TelnetState::SubnegotiationIac;
                    }
                }
                TelnetState::SubnegotiationIac => {
                    self.state = if byte == SE {
                        TelnetState::Data
                    } else {
                        TelnetState::Subnegotiation
                    };
                }
            }
        }
        (data, reply)
    }
}

fn negotiation_reply(command: u8, option: u8) -> [u8; 3] {
    let response = match command {
        DO => {
            if matches!(option, OPT_SGA | OPT_NAWS) {
                WILL
            } else {
                WONT
            }
        }
        DONT => WONT,
        WILL => {
            if matches!(option, OPT_ECHO | OPT_SGA) {
                DO
            } else {
                DONT
            }
        }
        WONT => DONT,
        _ => DONT,
    };
    [IAC, response, option]
}

fn escape_iac(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    for &byte in input {
        output.push(byte);
        if byte == IAC {
            output.push(IAC);
        }
    }
    output
}

fn naws(cols: u16, rows: u16) -> Vec<u8> {
    let mut output = vec![IAC, SB, OPT_NAWS];
    for byte in cols
        .max(1)
        .to_be_bytes()
        .into_iter()
        .chain(rows.max(1).to_be_bytes())
    {
        output.push(byte);
        if byte == IAC {
            output.push(IAC);
        }
    }
    output.extend([IAC, SE]);
    output
}

#[cfg(test)]
mod tests {
    use super::{DO, IAC, OPT_NAWS, TelnetParser, escape_iac, naws};

    #[test]
    fn parser_removes_negotiation_and_preserves_escaped_iac() {
        let mut parser = TelnetParser::default();
        let (data, reply) = parser.process(&[b'a', IAC, IAC, b'b', IAC, DO, OPT_NAWS]);

        assert_eq!(data, vec![b'a', IAC, b'b']);
        assert_eq!(reply, vec![IAC, 251, OPT_NAWS]);
    }

    #[test]
    fn input_and_naws_escape_iac_bytes() {
        assert_eq!(escape_iac(&[1, IAC, 2]), vec![1, IAC, IAC, 2]);
        assert_eq!(
            naws(255, 24),
            vec![IAC, 250, OPT_NAWS, 0, IAC, IAC, 0, 24, IAC, 240]
        );
    }
}
