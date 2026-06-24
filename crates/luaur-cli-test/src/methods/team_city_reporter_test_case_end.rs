use crate::records::team_city_reporter::TeamCityReporter;
use core::ffi::{c_char, c_void, CStr};
use std::io::Write;

// EXTERNAL_CRATE_REQUIRED: doctest - used for CurrentTestCaseStats type
// The doctest::CurrentTestCaseStats type is not translated; we treat it as an opaque pointer
// or assume the C++ ABI layout for accessing fields.

pub fn team_city_reporter_test_case_end(reporter: &TeamCityReporter, in_stats: *const c_void) {
    unsafe {
        // doctest::CurrentTestCaseStats layout (approximate):
        // struct CurrentTestCaseStats {
        //     int numAssertsCurrentTest;
        //     int numAssertsFailedCurrentTest;
        //     double seconds;
        //     bool testCaseSuccess;
        // };
        let num_asserts = *(in_stats as *const i32);
        let failed_asserts = *(in_stats.add(4) as *const i32);
        let seconds = *(in_stats.add(8) as *const f64);
        let success = *(in_stats.add(16) as *const bool);

        // TeamCityReporter::currentTest layout (from previous context):
        // struct TestCase {
        //     const char* m_file;
        //     int m_line;
        //     const char* m_test_suite;
        //     const char* m_name;
        // };
        let tc_ptr = reporter.current_test as *const *const c_char;
        let suite_ptr = *tc_ptr.add(2);
        let name_ptr = *tc_ptr.add(3);

        let suite = if !suite_ptr.is_null() {
            CStr::from_ptr(suite_ptr).to_string_lossy()
        } else {
            std::borrow::Cow::Borrowed("")
        };
        let name = if !name_ptr.is_null() {
            CStr::from_ptr(name_ptr).to_string_lossy()
        } else {
            std::borrow::Cow::Borrowed("")
        };

        println!(
            "##teamcity[testMetadata testName='{}: {}' name='total_asserts' type='number' value='{}']",
            suite, name, num_asserts
        );
        println!(
            "##teamcity[testMetadata testName='{}: {}' name='failed_asserts' type='number' value='{}']",
            suite, name, failed_asserts
        );
        println!(
            "##teamcity[testMetadata testName='{}: {}' name='runtime' type='number' value='{}']",
            suite, name, seconds
        );

        if !success {
            println!("##teamcity[testFailed name='{}: {}']", suite, name);
        }

        println!("##teamcity[testFinished name='{}: {}']", suite, name);
    }

    let _ = std::io::stdout().flush();
}
