//! Raw serial-console backend for terminal tabs.

use std::{
    io::{Read, Write},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Sender},
    },
    thread,
    time::Duration,
};

use anyhow::{Context, Result};
use serialport::{DataBits, FlowControl, Parity, StopBits};

use crate::{
    events::{BackendEvent, BackendEventSender},
    session::Session,
    terminal::{BackendCommand, BackendShutdown, BackendTx},
};

const SERIAL_READ_TIMEOUT: Duration = Duration::from_millis(50);

struct SerialBackendShutdown {
    commands: Sender<BackendCommand>,
    worker: Mutex<Option<thread::JoinHandle<()>>>,
    shutdown_requested: Arc<AtomicBool>,
}

impl BackendShutdown for SerialBackendShutdown {
    fn shutdown(&self) {
        if self.shutdown_requested.swap(true, Ordering::SeqCst) {
            return;
        }

        let _ = self.commands.send(BackendCommand::Close);
        let worker = self
            .worker
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take();
        let Some(worker) = worker else {
            return;
        };
        thread::spawn(move || {
            let _ = worker.join();
        });
    }
}

/// Enumerate ports only when the connection form opens or the user refreshes it.
pub(crate) fn available_port_names() -> Vec<String> {
    match serialport::available_ports() {
        Ok(ports) => {
            let mut names = ports
                .into_iter()
                .map(|port| port.port_name)
                .collect::<Vec<_>>();
            names.sort_unstable();
            names.dedup();
            names
        }
        Err(err) => {
            tracing::warn!(
                component = "serial",
                operation = "enumerate_ports",
                error = %crate::diagnostics::sanitize_error(&err.to_string()),
                "Could not enumerate serial ports"
            );
            Vec::new()
        }
    }
}

/// Start a serial session without opening the device on the UI thread.
pub(crate) fn spawn_serial_terminal(
    tab_id: String,
    session: Session,
    events: BackendEventSender,
) -> BackendTx {
    let (commands, receiver) = mpsc::channel::<BackendCommand>();
    let shutdown_requested = Arc::new(AtomicBool::new(false));
    let worker_shutdown_requested = shutdown_requested.clone();
    let worker_events = events.clone();
    let worker_tab_id = tab_id.clone();

    let _ = events.try_send(BackendEvent::Status {
        tab_id: tab_id.clone(),
        text: format!("opening serial port {}", session.serial_port.trim()),
    });
    let worker = thread::spawn(move || {
        let result = run_serial_terminal(
            &worker_tab_id,
            session,
            receiver,
            worker_events.clone(),
            worker_shutdown_requested.clone(),
        );
        if let Err(err) = result
            && !worker_shutdown_requested.load(Ordering::SeqCst)
        {
            let _ = worker_events.blocking_send(BackendEvent::Closed {
                tab_id: worker_tab_id,
                reason: format!("serial error: {err:#}"),
            });
        }
    });

    BackendTx::Serial {
        commands: commands.clone(),
        shutdown: Arc::new(SerialBackendShutdown {
            commands,
            worker: Mutex::new(Some(worker)),
            shutdown_requested,
        }),
    }
}

fn run_serial_terminal(
    tab_id: &str,
    session: Session,
    receiver: mpsc::Receiver<BackendCommand>,
    events: BackendEventSender,
    shutdown_requested: Arc<AtomicBool>,
) -> Result<()> {
    let port_name = session.serial_port.trim().to_string();
    if port_name.is_empty() {
        anyhow::bail!("serial port is required");
    }

    let port = serialport::new(&port_name, session.baud_rate.max(1))
        .data_bits(data_bits(session.data_bits))
        .stop_bits(stop_bits(session.stop_bits))
        .parity(parity(&session.parity))
        .flow_control(flow_control(&session.flow_control))
        .timeout(SERIAL_READ_TIMEOUT)
        .open()
        .with_context(|| format!("open serial port {port_name}"))?;
    let writer = Arc::new(Mutex::new(
        port.try_clone().context("clone serial port for writer")?,
    ));

    let _ = events.blocking_send(BackendEvent::Connected {
        tab_id: tab_id.to_string(),
    });
    let _ = events.blocking_send(BackendEvent::Status {
        tab_id: tab_id.to_string(),
        text: format!(
            "serial connected: {} @ {} {}{}{}",
            port_name,
            session.baud_rate,
            session.data_bits,
            parity_letter(&session.parity),
            session.stop_bits
        ),
    });

    let reader_events = events.clone();
    let reader_tab_id = tab_id.to_string();
    let reader_shutdown_requested = shutdown_requested.clone();
    let reader = thread::spawn(move || {
        let mut port = port;
        let mut buffer = [0u8; 4096];
        while !reader_shutdown_requested.load(Ordering::SeqCst) {
            match port.read(&mut buffer) {
                Ok(0) => {}
                Ok(read) => {
                    if reader_events
                        .blocking_send(BackendEvent::Output {
                            tab_id: reader_tab_id.clone(),
                            bytes: buffer[..read].to_vec(),
                        })
                        .is_err()
                    {
                        return;
                    }
                }
                Err(err)
                    if matches!(
                        err.kind(),
                        std::io::ErrorKind::TimedOut | std::io::ErrorKind::Interrupted
                    ) => {}
                Err(err) => {
                    if !reader_shutdown_requested.load(Ordering::SeqCst) {
                        let _ = reader_events.blocking_send(BackendEvent::Closed {
                            tab_id: reader_tab_id,
                            reason: format!("serial read error: {err}"),
                        });
                    }
                    return;
                }
            }
        }
    });

    while let Ok(command) = receiver.recv() {
        match command {
            BackendCommand::Input(bytes) => {
                let result = {
                    let mut writer = writer
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner());
                    writer.write_all(&bytes).and_then(|_| writer.flush())
                };
                if let Err(err) = result {
                    if !shutdown_requested.load(Ordering::SeqCst) {
                        let _ = events.blocking_send(BackendEvent::Closed {
                            tab_id: tab_id.to_string(),
                            reason: format!("serial write error: {err}"),
                        });
                    }
                    break;
                }
            }
            BackendCommand::Close => break,
            BackendCommand::Resize { .. }
            | BackendCommand::SampleMetrics { .. }
            | BackendCommand::CheckConnection { .. }
            | BackendCommand::QueryWorkingDirectory => {}
        }
    }
    shutdown_requested.store(true, Ordering::SeqCst);
    let _ = reader.join();
    Ok(())
}

fn data_bits(value: u8) -> DataBits {
    match value {
        5 => DataBits::Five,
        6 => DataBits::Six,
        7 => DataBits::Seven,
        _ => DataBits::Eight,
    }
}

fn stop_bits(value: u8) -> StopBits {
    if value == 2 {
        StopBits::Two
    } else {
        StopBits::One
    }
}

fn parity(value: &str) -> Parity {
    match value {
        "odd" => Parity::Odd,
        "even" => Parity::Even,
        _ => Parity::None,
    }
}

fn parity_letter(value: &str) -> &'static str {
    match value {
        "odd" => "O",
        "even" => "E",
        _ => "N",
    }
}

fn flow_control(value: &str) -> FlowControl {
    match value {
        "hardware" => FlowControl::Hardware,
        "software" => FlowControl::Software,
        _ => FlowControl::None,
    }
}

#[cfg(test)]
mod tests {
    use serialport::{DataBits, FlowControl, Parity, StopBits};

    use super::{data_bits, flow_control, parity, stop_bits};

    #[test]
    fn serial_parameters_fall_back_to_8n1_without_flow_control() {
        assert_eq!(data_bits(42), DataBits::Eight);
        assert_eq!(stop_bits(42), StopBits::One);
        assert_eq!(parity("unexpected"), Parity::None);
        assert_eq!(flow_control("unexpected"), FlowControl::None);
    }
}
