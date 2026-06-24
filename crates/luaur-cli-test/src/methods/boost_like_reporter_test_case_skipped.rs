use crate::records::boost_like_reporter::BoostLikeReporter;

pub fn boost_like_reporter_test_case_skipped(
    _self: &mut BoostLikeReporter,
    _data: *const core::ffi::c_void,
) {
    // Native-only; no-op implementation
}
