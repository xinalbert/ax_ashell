use std::time::Instant;

use gpui::SharedString;

use crate::monitoring::{SystemSampler, SystemSnapshot};

pub(crate) struct MonitoringState {
    pub(crate) status: Option<SharedString>,
    /// Local system sampling is only needed while a local monitoring view is visible.
    pub(crate) sampler: Option<SystemSampler>,
    pub(crate) system: SystemSnapshot,
    pub(crate) cpu_history: Vec<f32>,
    pub(crate) net_rx_history: Vec<f32>,
    pub(crate) net_tx_history: Vec<f32>,
    pub(crate) last_sample: Instant,
    pub(crate) system_tab_id: Option<String>,
    pub(crate) remote_sample_generation: u64,
    pub(crate) remote_sample_in_flight: Option<(u64, bool)>,
}

impl MonitoringState {
    pub(crate) fn begin_remote_sample(&mut self, is_resume_health_check: bool) -> Option<u64> {
        if self.remote_sample_in_flight.is_some() {
            return None;
        }

        let generation = self.remote_sample_generation;
        self.remote_sample_in_flight = Some((generation, is_resume_health_check));
        Some(generation)
    }

    pub(crate) fn invalidate_remote_samples(&mut self) {
        self.remote_sample_generation = self.remote_sample_generation.wrapping_add(1);
        self.remote_sample_in_flight = None;
    }

    pub(crate) fn finish_remote_sample(&mut self, generation: u64) -> Option<bool> {
        let (in_flight_generation, is_resume_health_check) = self.remote_sample_in_flight?;
        if in_flight_generation != generation {
            return None;
        }

        self.remote_sample_in_flight = None;
        Some(is_resume_health_check)
    }
}

#[cfg(test)]
mod tests {
    use super::MonitoringState;

    fn monitoring_state() -> MonitoringState {
        MonitoringState {
            status: None,
            sampler: None,
            system: Default::default(),
            cpu_history: Vec::new(),
            net_rx_history: Vec::new(),
            net_tx_history: Vec::new(),
            last_sample: std::time::Instant::now(),
            system_tab_id: None,
            remote_sample_generation: 0,
            remote_sample_in_flight: None,
        }
    }

    #[test]
    fn resume_invalidates_an_older_remote_sample() {
        let mut monitoring = monitoring_state();
        let first = monitoring
            .begin_remote_sample(false)
            .expect("first sample starts");

        monitoring.invalidate_remote_samples();

        assert_eq!(monitoring.finish_remote_sample(first), None);
        let resumed = monitoring
            .begin_remote_sample(true)
            .expect("resumed sample starts");
        assert_ne!(first, resumed);
        assert_eq!(monitoring.finish_remote_sample(resumed), Some(true));
    }
}
