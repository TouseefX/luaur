extern crate alloc;

use alloc::ffi::CString;
use core::ffi;

#[derive(Debug, Clone)]
pub struct TeamCityReporter {
    pub(crate) current_test: *const ffi::c_void,
}

impl TeamCityReporter {
    pub fn team_city_reporter_team_city_reporter() -> Self {
        Self {
            current_test: core::ptr::null_mut(),
        }
    }
}
