use std::sync::{Arc, mpsc::Sender};

#[derive(Debug)]
pub enum BackendCommand {
    Input(Vec<u8>),
    Resize {
        cols: u16,
        rows: u16,
    },
    SampleMetrics {
        generation: u64,
    },
    CheckConnection {
        generation: u64,
        backend_generation: u32,
    },
    QueryWorkingDirectory,
    Close,
}

pub trait BackendShutdown: Send + Sync {
    /// Start a non-blocking, backend-specific shutdown and resource reap.
    fn shutdown(&self);
}

struct InactiveBackendShutdown;

impl BackendShutdown for InactiveBackendShutdown {
    fn shutdown(&self) {}
}

#[derive(Clone)]
pub enum BackendTx {
    Local {
        commands: Sender<BackendCommand>,
        shutdown: std::sync::Arc<dyn BackendShutdown>,
    },
    Ssh {
        commands: tokio::sync::mpsc::UnboundedSender<BackendCommand>,
        shutdown: std::sync::Arc<dyn BackendShutdown>,
    },
    Serial {
        commands: Sender<BackendCommand>,
        shutdown: std::sync::Arc<dyn BackendShutdown>,
    },
    Telnet {
        commands: tokio::sync::mpsc::UnboundedSender<BackendCommand>,
        shutdown: std::sync::Arc<dyn BackendShutdown>,
    },
}

impl BackendTx {
    pub fn inactive() -> Self {
        let (commands, _receiver) = std::sync::mpsc::channel();
        Self::Local {
            commands,
            shutdown: Arc::new(InactiveBackendShutdown),
        }
    }

    pub fn send(&self, command: BackendCommand) {
        if matches!(command, BackendCommand::Close) {
            self.shutdown();
            return;
        }

        match self {
            Self::Local { commands, .. } | Self::Serial { commands, .. } => {
                let _ = commands.send(command);
            }
            Self::Ssh { commands, .. } | Self::Telnet { commands, .. } => {
                let _ = commands.send(command);
            }
        }
    }

    /// Signal the backend and schedule its resource reaper without blocking UI work.
    pub fn shutdown(&self) {
        match self {
            Self::Local { shutdown, .. }
            | Self::Ssh { shutdown, .. }
            | Self::Serial { shutdown, .. }
            | Self::Telnet { shutdown, .. } => shutdown.shutdown(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    };

    use super::{BackendCommand, BackendShutdown, BackendTx};

    struct CountingShutdown(AtomicUsize);

    impl BackendShutdown for CountingShutdown {
        fn shutdown(&self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn close_command_uses_the_backend_shutdown_controller() {
        let (commands, receiver) = mpsc::channel();
        let shutdown = Arc::new(CountingShutdown(AtomicUsize::new(0)));
        let backend = BackendTx::Local {
            commands,
            shutdown: shutdown.clone(),
        };

        backend.send(BackendCommand::Close);

        assert_eq!(shutdown.0.load(Ordering::SeqCst), 1);
        assert!(receiver.try_recv().is_err());
    }
}
