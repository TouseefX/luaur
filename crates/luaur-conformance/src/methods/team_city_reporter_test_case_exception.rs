use core::ffi;
use std::io::Write;

use crate::records::team_city_reporter::TeamCityReporter;

#[repr(C)]
struct TeamCityTestCaseException {
    error_string: alloc::string::String,
}

#[repr(C)]
struct TeamCityCurrentTest {
    m_test_suite: *const ffi::c_char,
    m_name: *const ffi::c_char,
}

pub fn team_city_reporter_test_case_exception(
    self_: &mut TeamCityReporter,
    in_data: *const ffi::c_void,
) {
    let current_test = unsafe { &*((*self_).current_test as *const TeamCityCurrentTest) };

    let exc = unsafe { &*(in_data as *const TeamCityTestCaseException) };
    let details = exc.error_string.as_str();

    let suite = unsafe { ffi::CStr::from_ptr(current_test.m_test_suite).to_string_lossy() };
    let name = unsafe { ffi::CStr::from_ptr(current_test.m_name).to_string_lossy() };

    println!(
        "##teamcity[testFailed name='{}: {}' message='Unhandled exception' details='{}']",
        suite, name, details
    );

    let _ = std::io::stdout().flush();
}
