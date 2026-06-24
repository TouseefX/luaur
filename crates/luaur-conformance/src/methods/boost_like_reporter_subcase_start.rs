use crate::records::boost_like_reporter::BoostLikeReporter;
use core::ffi::c_void;

impl BoostLikeReporter {
    pub fn subcase_start(&mut self, _signature: *const c_void) {}
}
