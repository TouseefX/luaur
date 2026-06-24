use crate::records::boost_like_reporter::BoostLikeReporter;

impl BoostLikeReporter {
    pub fn new(_in: *const core::ffi::c_void) -> Self {
        Self {
            current_test: core::ptr::null(),
        }
    }
}

#[allow(non_snake_case)]
pub fn boost_like_reporter_boost_like_reporter(
    this: *mut BoostLikeReporter,
    in_: *const core::ffi::c_void,
) {
    unsafe {
        (*this).current_test = core::ptr::null();
    }
}
