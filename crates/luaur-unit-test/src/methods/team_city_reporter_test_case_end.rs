extern crate luaur_common;

use core::ffi::CStr;
use std::io::Write;

impl crate::records::team_city_reporter::TeamCityReporter {
    pub fn test_case_end(
        &mut self,
        in_stats: &crate::methods::team_city_reporter_test_case_end::doctest::CurrentTestCaseStats,
    ) {
        // Mirror the C++ behavior: print 3 TeamCity attributes and optionally a failed event.
        // The TestCaseData record is defined locally in this module to satisfy the pointer cast.
        let tc = self.current_test as *const crate::methods::team_city_reporter_test_case_end::records::test_case_data::TestCaseData;
        unsafe {
            let suite = CStr::from_ptr((*tc).m_test_suite).to_string_lossy();
            let name = CStr::from_ptr((*tc).m_name).to_string_lossy();

            print!(
                "##teamcity[testMetadata testName='{}: {}' name='total_asserts' type='number' value='{}']\n",
                suite, name, in_stats.numAssertsCurrentTest
            );
            print!(
                "##teamcity[testMetadata testName='{}: {}' name='failed_asserts' type='number' value='{}']\n",
                suite, name, in_stats.numAssertsFailedCurrentTest
            );
            print!(
                "##teamcity[testMetadata testName='{}: {}' name='runtime' type='number' value='{}']\n",
                suite, name, in_stats.seconds
            );

            if !in_stats.testCaseSuccess {
                print!("##teamcity[testFailed name='{}: {}']\n", suite, name);
            }

            print!("##teamcity[testFinished name='{}: {}']\n", suite, name);
            let _ = std::io::stdout().flush();
        }
    }
}

#[allow(non_camel_case_types)]
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
