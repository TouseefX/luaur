#[derive(Debug, Clone)]
pub struct BoostLikeReporter {
    pub(crate) current_test: *const core::ffi::c_void,
}
