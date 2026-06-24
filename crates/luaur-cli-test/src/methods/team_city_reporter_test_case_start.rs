use crate::records::team_city_reporter::TeamCityReporter;
use core::ffi::{c_char, c_void, CStr};
use std::io::Write;

// EXTERNAL_CRATE_REQUIRED: doctest - used for TestCaseData type
// The doctest::TestCaseData type is not translated; we treat it as an opaque pointer
// or assume the C++ ABI layout for accessing fields.

pub fn team_city_reporter_test_case_start(reporter: &mut TeamCityReporter, in_data: *const c_void) {
    reporter.current_test = in_data;

    unsafe {
        // doctest::TestCaseData layout (approximate):
        // struct TestCaseData {
        //     const char* m_test_suite;
        //     const char* m_name;
        //     ...
        // };
        // We assume m_test_suite is at offset 0 and m_name is at offset 8 (on 64-bit).
        let suite_ptr = *(in_data as *const *const c_char);
        let name_ptr = *((in_data as *const u8).add(8) as *const *const c_char);

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
            "##teamcity[testStarted name='{}: {}' captureStandardOutput='true']",
            suite, name
        );
    }

    let _ = std::io::stdout().flush();
}
