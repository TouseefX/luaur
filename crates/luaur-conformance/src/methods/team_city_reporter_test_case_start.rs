use crate::records::team_city_reporter::TeamCityReporter;
use core::ffi::{c_char, c_void, CStr};
use std::io::Write;

impl TeamCityReporter {
    pub fn test_case_start_impl(&mut self, in_data: &c_void) {
        let tc = in_data as *const c_void as *const TestCaseData;
        self.current_test = in_data as *const c_void;

        let test_suite = unsafe { CStr::from_ptr((*tc).m_test_suite) }.to_string_lossy();
        let name = unsafe { CStr::from_ptr((*tc).m_name) }.to_string_lossy();

        println!(
            "##teamcity[testStarted name='{}: {}' captureStandardOutput='true']",
            test_suite, name
        );
        let _ = std::io::stdout().flush();
    }
}

#[repr(C)]
pub struct TestCaseData {
    pub m_test_suite: *const c_char,
    pub m_name: *const c_char,
    pub m_type: *const c_char,
    pub m_file: *const c_char,
    pub m_line: u32,
    pub m_skipped: bool,
    pub m_no_breaks: bool,
    pub m_no_output: bool,
    pub m_may_fail: bool,
    pub m_should_fail: bool,
    pub m_expected_failures: i32,
    pub m_timeout: f64,
}
