use crate::records::boost_like_reporter::BoostLikeReporter;
use core::ffi::c_void;

impl BoostLikeReporter {
    pub fn test_case_skipped(&mut self, _test_case_data: *const c_void) {}
}
