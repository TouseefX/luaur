#[derive(Debug)]
#[repr(C)]
pub struct TeamCityReporter {
    pub(crate) current_test: *const core::ffi::c_void,
}
