use crate::records::boost_like_reporter::BoostLikeReporter;
use core::ffi::c_void;

impl BoostLikeReporter {
    pub fn test_run_end(&mut self, _ts: *const c_void) {
        // Native-only; stub implementation
    }
}
