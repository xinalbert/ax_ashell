use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

use tokio::runtime::{Builder, Handle, Runtime};

use crate::events::{BackendEvent, BackendEventSender};

const RUNTIME_WORKER_THREADS: usize = 2;
const RUNTIME_MAX_BLOCKING_THREADS: usize = 8;
pub(crate) const DEFAULT_FOREGROUND_FRAME_INTERVAL: Duration = Duration::from_millis(16);
const MIN_FOREGROUND_FRAME_INTERVAL: Duration = Duration::from_micros(8_333);
const FOREGROUND_ACTIVITY_WINDOW: Duration = Duration::from_millis(50);
const FRAME_CADENCE_MAX_AGE: Duration = Duration::from_secs(2);
const FRAME_CALIBRATION_FRAMES: u8 = 3;
const MIN_SAMPLE_INTERVAL: Duration = Duration::from_millis(2);
const MAX_SAMPLE_INTERVAL: Duration = Duration::from_millis(100);

#[derive(Debug)]
struct FrameCadence {
    refresh_interval: Option<Duration>,
    sampled_at: Option<Instant>,
    last_activity_at: Option<Instant>,
    calibration: Option<FrameCalibration>,
}

#[derive(Debug)]
struct FrameCalibration {
    frames_seen: u8,
    last_frame_at: Option<Instant>,
    total_interval: Duration,
    intervals_seen: u8,
}

impl FrameCadence {
    fn note_activity(&mut self, now: Instant) {
        self.last_activity_at = Some(now);
        if self.calibration.is_some()
            || self
                .sampled_at
                .is_some_and(|sampled_at| now.duration_since(sampled_at) < FRAME_CADENCE_MAX_AGE)
        {
            return;
        }

        self.calibration = Some(FrameCalibration {
            frames_seen: 0,
            last_frame_at: None,
            total_interval: Duration::ZERO,
            intervals_seen: 0,
        });
    }

    fn record_frame(&mut self, now: Instant) -> bool {
        let Some(calibration) = self.calibration.as_mut() else {
            return false;
        };

        if let Some(last_frame_at) = calibration.last_frame_at {
            let interval = now.saturating_duration_since(last_frame_at);
            if (MIN_SAMPLE_INTERVAL..=MAX_SAMPLE_INTERVAL).contains(&interval) {
                calibration.total_interval = calibration.total_interval.saturating_add(interval);
                calibration.intervals_seen = calibration.intervals_seen.saturating_add(1);
            }
        }
        calibration.last_frame_at = Some(now);
        calibration.frames_seen = calibration.frames_seen.saturating_add(1);

        if calibration.frames_seen < FRAME_CALIBRATION_FRAMES {
            return true;
        }

        let calibration = self.calibration.take().expect("frame calibration exists");
        self.sampled_at = Some(now);
        self.refresh_interval = (calibration.intervals_seen == FRAME_CALIBRATION_FRAMES - 1)
            .then(|| calibration.total_interval / u32::from(calibration.intervals_seen))
            .map(|interval| {
                interval.clamp(
                    MIN_FOREGROUND_FRAME_INTERVAL,
                    DEFAULT_FOREGROUND_FRAME_INTERVAL,
                )
            });
        false
    }

    fn refresh_interval(&self, now: Instant) -> Duration {
        self.is_activity_recent(now)
            .then_some(
                self.refresh_interval
                    .unwrap_or(DEFAULT_FOREGROUND_FRAME_INTERVAL),
            )
            .unwrap_or(DEFAULT_FOREGROUND_FRAME_INTERVAL)
    }

    fn is_activity_recent(&self, now: Instant) -> bool {
        self.last_activity_at.is_some_and(|last_activity_at| {
            now.duration_since(last_activity_at) < FOREGROUND_ACTIVITY_WINDOW
        })
    }

    fn reset(&mut self) {
        self.refresh_interval = None;
        self.sampled_at = None;
        self.last_activity_at = None;
        self.calibration = None;
    }
}

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
    frame_cadence: FrameCadence,
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
            frame_cadence: FrameCadence {
                refresh_interval: None,
                sampled_at: None,
                last_activity_at: None,
                calibration: None,
            },
        }
    }

    pub(crate) fn note_foreground_refresh_activity(&mut self, now: Instant) {
        self.frame_cadence.note_activity(now);
    }

    pub(crate) fn record_foreground_frame(&mut self, now: Instant) -> bool {
        self.frame_cadence.record_frame(now)
    }

    pub(crate) fn foreground_refresh_interval(&self, now: Instant) -> Duration {
        self.frame_cadence.refresh_interval(now)
    }

    pub(crate) fn reset_frame_cadence(&mut self) {
        self.frame_cadence.reset();
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
    use std::time::{Duration, Instant};

    use crate::events::backend_event_channel;

    use super::{DEFAULT_FOREGROUND_FRAME_INTERVAL, RuntimeState};

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

    #[test]
    fn frame_cadence_uses_a_three_frame_120hz_calibration() {
        let mut state = runtime_state();
        let start = Instant::now();

        state.note_foreground_refresh_activity(start);
        assert!(state.record_foreground_frame(start));
        assert!(state.record_foreground_frame(start + Duration::from_micros(8_333)));
        assert!(!state.record_foreground_frame(start + Duration::from_micros(16_666)));
        assert_eq!(
            state.foreground_refresh_interval(start + Duration::from_micros(16_666)),
            Duration::from_micros(8_333)
        );
    }

    #[test]
    fn frame_cadence_preserves_the_60hz_default_for_slower_samples() {
        let mut state = runtime_state();
        let start = Instant::now();

        state.note_foreground_refresh_activity(start);
        assert!(state.record_foreground_frame(start));
        assert!(state.record_foreground_frame(start + Duration::from_millis(33)));
        assert!(!state.record_foreground_frame(start + Duration::from_millis(66)));
        assert_eq!(
            state.foreground_refresh_interval(start + Duration::from_millis(66)),
            DEFAULT_FOREGROUND_FRAME_INTERVAL
        );
    }

    #[test]
    fn frame_cadence_caps_faster_displays_at_120hz() {
        let mut state = runtime_state();
        let start = Instant::now();

        state.note_foreground_refresh_activity(start);
        assert!(state.record_foreground_frame(start));
        assert!(state.record_foreground_frame(start + Duration::from_millis(4)));
        assert!(!state.record_foreground_frame(start + Duration::from_millis(8)));
        assert_eq!(
            state.foreground_refresh_interval(start + Duration::from_millis(8)),
            Duration::from_micros(8_333)
        );
    }

    #[test]
    fn frame_cadence_stays_idle_until_new_activity_and_resets_on_inactivity() {
        let mut state = runtime_state();
        let start = Instant::now();

        assert!(!state.record_foreground_frame(start));
        state.note_foreground_refresh_activity(start);
        state.reset_frame_cadence();
        assert!(!state.record_foreground_frame(start + Duration::from_millis(1)));
        assert_eq!(
            state.foreground_refresh_interval(start + Duration::from_millis(1)),
            DEFAULT_FOREGROUND_FRAME_INTERVAL
        );
    }

    #[test]
    fn frame_cadence_returns_to_60hz_after_activity_stops() {
        let mut state = runtime_state();
        let start = Instant::now();

        state.note_foreground_refresh_activity(start);
        assert!(state.record_foreground_frame(start));
        assert!(state.record_foreground_frame(start + Duration::from_micros(8_333)));
        assert!(!state.record_foreground_frame(start + Duration::from_micros(16_666)));
        assert_eq!(
            state.foreground_refresh_interval(start + Duration::from_millis(51)),
            DEFAULT_FOREGROUND_FRAME_INTERVAL
        );
    }

    #[test]
    fn frame_cadence_keeps_120hz_while_activity_continues() {
        let mut state = runtime_state();
        let start = Instant::now();

        state.note_foreground_refresh_activity(start);
        assert!(state.record_foreground_frame(start));
        assert!(state.record_foreground_frame(start + Duration::from_micros(8_333)));
        assert!(!state.record_foreground_frame(start + Duration::from_micros(16_666)));
        state.note_foreground_refresh_activity(start + Duration::from_millis(49));
        assert_eq!(
            state.foreground_refresh_interval(start + Duration::from_millis(51)),
            Duration::from_micros(8_333)
        );
    }
}
