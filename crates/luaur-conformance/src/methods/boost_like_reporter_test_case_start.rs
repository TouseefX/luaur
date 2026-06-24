use crate::records::boost_like_reporter::BoostLikeReporter;
use std::io::Write;

impl BoostLikeReporter {
    pub fn test_case_start(&mut self, tc: &doctest::TestCaseData) {
        self.current_test = tc as *const doctest::TestCaseData as *const core::ffi::c_void;

        let test_suite = unsafe { core::ffi::CStr::from_ptr(tc.m_test_suite) }.to_string_lossy();
        let name = unsafe { core::ffi::CStr::from_ptr(tc.m_name) }.to_string_lossy();

        println!("Entering test suite \"{}\"", test_suite);
        println!("Entering test case \"{}\"", name);

        let _ = std::io::stdout().flush();
    }
}

/// This module provides a minimal representation of the doctest::TestCaseData structure
/// as used by the conformance test reporter.
pub mod doctest {
    use core::ffi::c_char;

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
}
