use core::ffi::c_void;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct BoostLikeReporter {
    pub(crate) current_test: *const c_void,
}

impl BoostLikeReporter {
    pub fn boost_like_reporter(&mut self) {
        self.current_test = core::ptr::null();
    }
}
