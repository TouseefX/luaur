use crate::records::team_city_reporter::TeamCityReporter;

pub fn team_city_reporter_test_run_end(
    _self: &mut TeamCityReporter,
    _in: *const core::ffi::c_void,
) {
    // Native-only; doctest test_run_end hook
}
