extern crate luaur_common;

use core::ffi::CStr;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use std::io::Write;

impl crate::records::boost_like_reporter::BoostLikeReporter {
    pub fn test_case_end(&mut self, _tc_num_asserts: i32) {
        LUAU_ASSERT!(!self.current_test.is_null());

        // Note: In this codebase, the test case data structure is represented by TestCaseData.
        // We cast the opaque current_test pointer back to its concrete type to access the names.
        // The type is defined locally in this module to satisfy the cast, mirroring the
        // pattern used in TeamCityReporter::test_case_end.
        let tc = self.current_test as *const self::records::test_case_data::TestCaseData;
        unsafe {
            let name = CStr::from_ptr((*tc).m_name).to_string_lossy();
            let suite = CStr::from_ptr((*tc).m_test_suite).to_string_lossy();
            println!("Leaving test case \"{}\"", name);
            println!("Leaving test suite \"{}\"", suite);
        }

        self.current_test = core::ptr::null();
        let _ = std::io::stdout().flush();
    }
}

#[allow(non_snake_case)]
pub mod doctest {
    #[repr(C)]
    pub struct CurrentTestCaseStats {
        pub numAssertsCurrentTest: i32,
        pub numAssertsFailedCurrentTest: i32,
        pub seconds: f64,
        pub testCaseSuccess: bool,
    }
}

pub mod records {
    pub mod test_case_data {
        #[repr(C)]
        pub struct TestCaseData {
            pub m_test_suite: *const core::ffi::c_char,
            pub m_name: *const core::ffi::c_char,
        }
    }
}
