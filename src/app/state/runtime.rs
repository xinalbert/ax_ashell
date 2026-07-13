use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Instant,
};

use tokio::runtime::{Builder, Handle, Runtime};

use crate::events::{BackendEvent, BackendEventSender};

const RUNTIME_WORKER_THREADS: usize = 2;
const RUNTIME_MAX_BLOCKING_THREADS: usize = 8;

#[derive(Clone)]
pub(crate) struct RuntimeTaskTracker(Arc<AtomicUsize>);

impl RuntimeTaskTracker {
    pub(crate) fn new() -> Self {
        Self(Arc::new(AtomicUsize::new(0)))
    }

    pub(crate) fn acquire(&self) -> RuntimeTaskLease {
        self.0.fetch_add(1, Ordering::SeqCst);
        RuntimeTaskLease {
            tracker: self.clone(),
        }
    }

    fn active_tasks(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }
}

pub(crate) struct RuntimeTaskLease {
    tracker: RuntimeTaskTracker,
}

impl Drop for RuntimeTaskLease {
    fn drop(&mut self) {
        let previous = self.tracker.0.fetch_sub(1, Ordering::SeqCst);
        debug_assert!(previous > 0, "Tokio runtime task lease underflow");
    }
}

pub(crate) struct RuntimeState {
    runtime: Option<Runtime>,
    task_tracker: RuntimeTaskTracker,
    pub(crate) events_rx: tokio::sync::mpsc::Receiver<BackendEvent>,
    pub(crate) events_tx: BackendEventSender,
    pub(crate) pending_terminal_refresh: bool,
    pub(crate) last_terminal_refresh: Instant,
    pub(crate) pending_ui_refresh: bool,
    pub(crate) last_ui_refresh: Instant,
    pub(crate) last_sftp_idle_sweep: Instant,
}

impl RuntimeState {
    pub(crate) fn new(
        events_rx: tokio::sync::mpsc::Receiver<BackendEvent>,
        events_tx: BackendEventSender,
    ) -> Self {
        Self {
            runtime: None,
            task_tracker: RuntimeTaskTracker::new(),
            events_rx,
            events_tx,
            pending_terminal_refresh: false,
            last_terminal_refresh: Instant::now(),
            pending_ui_refresh: false,
            last_ui_refresh: Instant::now(),
            last_sftp_idle_sweep: Instant::now(),
        }
    }

    pub(crate) fn runtime_handle_and_tracker(&mut self) -> (Handle, RuntimeTaskTracker) {
        let runtime = self.runtime.get_or_insert_with(|| {
            Builder::new_multi_thread()
                .worker_threads(RUNTIME_WORKER_THREADS)
                .max_blocking_threads(RUNTIME_MAX_BLOCKING_THREADS)
                .enable_all()
                .thread_name("ax-tokio")
                .build()
                .expect("create Tokio runtime")
        });
        (runtime.handle().clone(), self.task_tracker.clone())
    }

    pub(crate) fn release_runtime_if_idle(&mut self) -> bool {
        if self.task_tracker.active_tasks() != 0 {
            return false;
        }
        let Some(runtime) = self.runtime.take() else {
            return false;
        };
        runtime.shutdown_background();
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::events::backend_event_channel;

    use super::RuntimeState;

    fn runtime_state() -> RuntimeState {
        let (events_tx, events_rx) = backend_event_channel();
        RuntimeState::new(events_rx, events_tx)
    }

    #[test]
    fn runtime_is_created_on_demand_and_released_after_last_task() {
        let mut state = runtime_state();
        assert!(!state.release_runtime_if_idle());

        let (_, tasks) = state.runtime_handle_and_tracker();
        let task = tasks.acquire();
        assert!(!state.release_runtime_if_idle());

        drop(task);
        assert!(state.release_runtime_if_idle());
        assert!(!state.release_runtime_if_idle());
    }

    #[test]
    fn runtime_can_be_recreated_after_an_idle_release() {
        let mut state = runtime_state();

        let (_, tasks) = state.runtime_handle_and_tracker();
        let task = tasks.acquire();
        drop(task);
        assert!(state.release_runtime_if_idle());

        let (_, tasks) = state.runtime_handle_and_tracker();
        let task = tasks.acquire();
        assert!(!state.release_runtime_if_idle());

        drop(task);
        assert!(state.release_runtime_if_idle());
    }
}
