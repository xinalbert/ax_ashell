use std::time::{Duration, Instant, SystemTime};

const SYSTEM_RESUME_GAP: Duration = Duration::from_secs(10);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WindowLifecycleState {
    Foreground,
    Background,
    DeepSleep,
}

pub(crate) struct LifecycleState {
    state: WindowLifecycleState,
    inactive_since: Option<Instant>,
    last_event_pump_tick: Option<Instant>,
    last_wall_clock_tick: Option<SystemTime>,
}

impl LifecycleState {
    pub(crate) fn new(window_active: bool) -> Self {
        Self {
            state: if window_active {
                WindowLifecycleState::Foreground
            } else {
                WindowLifecycleState::Background
            },
            inactive_since: (!window_active).then(Instant::now),
            last_event_pump_tick: None,
            last_wall_clock_tick: None,
        }
    }

    pub(crate) fn state(&self) -> WindowLifecycleState {
        self.state
    }

    pub(crate) fn is_foreground(&self) -> bool {
        self.state == WindowLifecycleState::Foreground
    }

    pub(crate) fn set_window_active(&mut self, window_active: bool, now: Instant) -> bool {
        let next_state = if window_active {
            self.inactive_since = None;
            WindowLifecycleState::Foreground
        } else {
            self.inactive_since.get_or_insert(now);
            if self.state == WindowLifecycleState::DeepSleep {
                WindowLifecycleState::DeepSleep
            } else {
                WindowLifecycleState::Background
            }
        };

        let changed = self.state != next_state;
        self.state = next_state;
        changed
    }

    /// Detect a likely system resume without assuming whether the platform's
    /// monotonic clock advances while the machine is asleep.
    pub(crate) fn observe_event_pump_tick(&mut self, now: Instant, wall_clock: SystemTime) -> bool {
        let monotonic_gap = self
            .last_event_pump_tick
            .replace(now)
            .map(|previous| now.saturating_duration_since(previous))
            .unwrap_or_default();
        let wall_clock_gap = self
            .last_wall_clock_tick
            .replace(wall_clock)
            .and_then(|previous| wall_clock.duration_since(previous).ok())
            .unwrap_or_default();

        monotonic_gap >= SYSTEM_RESUME_GAP || wall_clock_gap >= SYSTEM_RESUME_GAP
    }

    pub(crate) fn advance(&mut self, now: Instant, deep_sleep_after_minutes: u32) -> bool {
        if deep_sleep_after_minutes == 0 {
            if self.state == WindowLifecycleState::DeepSleep {
                self.state = WindowLifecycleState::Background;
                return true;
            }
            return false;
        }

        if self.state != WindowLifecycleState::Background {
            return false;
        }

        let Some(inactive_since) = self.inactive_since else {
            return false;
        };
        let deep_sleep_after = Duration::from_secs(u64::from(deep_sleep_after_minutes) * 60);
        if now.duration_since(inactive_since) < deep_sleep_after {
            return false;
        }

        self.state = WindowLifecycleState::DeepSleep;
        true
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant, SystemTime};

    use super::{LifecycleState, WindowLifecycleState};

    #[test]
    fn inactive_window_enters_deep_sleep_after_configured_delay() {
        let now = Instant::now();
        let mut lifecycle = LifecycleState::new(true);

        assert!(lifecycle.set_window_active(false, now));
        assert_eq!(lifecycle.state(), WindowLifecycleState::Background);
        assert!(!lifecycle.advance(now + Duration::from_secs(59), 1));
        assert!(lifecycle.advance(now + Duration::from_secs(60), 1));
        assert_eq!(lifecycle.state(), WindowLifecycleState::DeepSleep);
    }

    #[test]
    fn disabled_deep_sleep_keeps_background_state() {
        let now = Instant::now();
        let mut lifecycle = LifecycleState::new(true);

        lifecycle.set_window_active(false, now);
        assert!(!lifecycle.advance(now + Duration::from_secs(3600), 0));
        assert_eq!(lifecycle.state(), WindowLifecycleState::Background);
    }

    #[test]
    fn activation_restores_foreground_from_deep_sleep() {
        let now = Instant::now();
        let mut lifecycle = LifecycleState::new(true);

        lifecycle.set_window_active(false, now);
        lifecycle.advance(now + Duration::from_secs(60), 1);
        assert!(lifecycle.set_window_active(true, now + Duration::from_secs(61)));
        assert_eq!(lifecycle.state(), WindowLifecycleState::Foreground);
        assert!(lifecycle.is_foreground());
    }

    #[test]
    fn repeated_deactivation_does_not_wake_deep_sleep() {
        let now = Instant::now();
        let mut lifecycle = LifecycleState::new(true);

        lifecycle.set_window_active(false, now);
        lifecycle.advance(now + Duration::from_secs(60), 1);
        assert!(!lifecycle.set_window_active(false, now + Duration::from_secs(61)));
        assert_eq!(lifecycle.state(), WindowLifecycleState::DeepSleep);
    }

    #[test]
    fn long_event_pump_gap_is_treated_as_a_possible_resume() {
        let now = Instant::now();
        let wall_clock = SystemTime::UNIX_EPOCH;
        let mut lifecycle = LifecycleState::new(true);

        assert!(!lifecycle.observe_event_pump_tick(now, wall_clock));
        assert!(!lifecycle.observe_event_pump_tick(
            now + Duration::from_secs(9),
            wall_clock + Duration::from_secs(9),
        ));
        assert!(lifecycle.observe_event_pump_tick(
            now + Duration::from_secs(19),
            wall_clock + Duration::from_secs(19),
        ));
        assert!(!lifecycle.observe_event_pump_tick(
            now + Duration::from_secs(19) + Duration::from_millis(1),
            wall_clock + Duration::from_secs(19) + Duration::from_millis(1),
        ));
    }

    #[test]
    fn wall_clock_gap_detects_resume_when_monotonic_time_does_not() {
        let now = Instant::now();
        let wall_clock = SystemTime::UNIX_EPOCH;
        let mut lifecycle = LifecycleState::new(true);

        assert!(!lifecycle.observe_event_pump_tick(now, wall_clock));
        assert!(lifecycle.observe_event_pump_tick(
            now + Duration::from_millis(1),
            wall_clock + Duration::from_secs(10),
        ));
    }
}
