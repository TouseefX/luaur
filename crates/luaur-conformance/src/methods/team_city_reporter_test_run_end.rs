use crate::records::team_city_reporter::TeamCityReporter;
use core::ffi::c_void;

impl TeamCityReporter {
    pub fn test_run_end_impl(&mut self, _stats: &c_void) {
        // Native-only; stub implementation
    }
}
