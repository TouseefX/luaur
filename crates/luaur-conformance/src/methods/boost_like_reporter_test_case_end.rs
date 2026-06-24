use crate::records::boost_like_reporter::BoostLikeReporter;
use core::ffi::{c_char, c_void, CStr};
use std::io::Write;

impl BoostLikeReporter {
    pub fn test_case_end(&mut self, tc: &doctest::CurrentTestCaseStats) {
        luaur_common::macros::luau_assert::LUAU_ASSERT!(!self.current_test.is_null());

        let name = unsafe { CStr::from_ptr((*tc).m_name) }.to_string_lossy();
        let test_suite = unsafe { CStr::from_ptr((*tc).m_test_suite) }.to_string_lossy();

        println!("Leaving test case \"{}\"", name);
        println!("Leaving test suite \"{}\"", test_suite);

        self.current_test = core::ptr::null();
        let _ = std::io::stdout().flush();
    }
}

/// This module provides a minimal representation of the doctest::CurrentTestCaseStats structure
/// as used by the conformance test reporter.
pub mod doctest {
    use core::ffi::c_char;

    #[repr(C)]
    pub struct CurrentTestCaseStats {
        pub m_name: *const c_char,
        pub m_test_suite: *const c_char,
    }
}
