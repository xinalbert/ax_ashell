use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WindowLifecycleState {
    Foreground,
    Background,
    DeepSleep,
}

pub(crate) struct LifecycleState {
    state: WindowLifecycleState,
    inactive_since: Option<Instant>,
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
    use std::time::{Duration, Instant};

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
}
