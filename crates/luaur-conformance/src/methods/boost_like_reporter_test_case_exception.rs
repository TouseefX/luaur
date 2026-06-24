use crate::records::boost_like_reporter::BoostLikeReporter;
use core::ffi::{c_char, c_void, CStr};
use std::io::Write;

#[macro_export]
macro_rules! LUAU_ASSERT {
    ($expr:expr) => {
        if !$expr {
            panic!("Assertion failed: {}", stringify!($expr));
        }
    };
}

impl BoostLikeReporter {
    pub fn test_case_exception(&self, e: &doctest::TestCaseException) {
        LUAU_ASSERT!(!self.current_test.is_null());

        let current_test = unsafe { &*(self.current_test as *const doctest::TestCaseData) };
        let file = unsafe { CStr::from_ptr(current_test.m_file).to_string_lossy() };
        let error = unsafe { CStr::from_ptr(e.error_string).to_string_lossy() };

        println!(
            "{}({}): FATAL: Unhandled exception {}",
            file, current_test.m_line, error
        );
        let _ = std::io::stdout().flush();
    }
}

pub mod doctest {
    use core::ffi::c_char;

    #[repr(C)]
    pub struct TestCaseException {
        pub error_string: *const c_char,
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
}
