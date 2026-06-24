use crate::records::team_city_reporter::TeamCityReporter;

impl TeamCityReporter {
    pub fn new(_in: *const core::ffi::c_void) -> Self {
        Self {
            current_test: core::ptr::null(),
        }
    }
}

#[allow(non_snake_case)]
pub fn team_city_reporter_team_city_reporter(
    this: &mut TeamCityReporter,
    _in: *const core::ffi::c_void,
) {
    this.current_test = core::ptr::null();
}
