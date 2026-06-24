#[derive(Debug)]
#[repr(C)]
pub struct BoostLikeReporter {
    pub(crate) current_test: *const core::ffi::c_void,
}
