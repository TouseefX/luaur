use crate::records::team_city_reporter::TeamCityReporter;
use core::ffi::{c_char, c_void, CStr};
use std::io::Write;

// EXTERNAL_CRATE_REQUIRED: doctest - used for TestCaseException type
// The doctest::TestCaseException type is not translated; we treat it as an opaque pointer
// and assume the C++ ABI layout for accessing fields based on the provided examples.

pub fn team_city_reporter_test_case_exception(reporter: &TeamCityReporter, in_: *const c_void) {
    // LUAU_ASSERT is not available here due to crate resolution issues; using standard assert.
    assert!(!reporter.current_test.is_null());

    unsafe {
        // doctest::TestCase layout (inferred from usage and sibling examples):
        // struct TestCase {
        //     const char* m_file;        // offset 0
        //     int m_line;                // offset 8
        //     const char* m_name;        // offset 16
        //     const char* m_test_suite;  // offset 24
        //     ...
        // };
        let tc_ptr = reporter.current_test as *const u8;
        let name_ptr = *(tc_ptr.add(16) as *const *const c_char);
        let test_suite_ptr = *(tc_ptr.add(24) as *const *const c_char);

        let name = if !name_ptr.is_null() {
            CStr::from_ptr(name_ptr).to_string_lossy()
        } else {
            std::borrow::Cow::Borrowed("")
        };

        let test_suite = if !test_suite_ptr.is_null() {
            CStr::from_ptr(test_suite_ptr).to_string_lossy()
        } else {
            std::borrow::Cow::Borrowed("")
        };

        // doctest::TestCaseException layout (from sibling example):
        // struct TestCaseException {
        //     std::string error_string; // offset 0
        //     bool is_crash;
        // };
        // std::string usually starts with a pointer to the buffer.
        let e_ptr = in_ as *const *const c_char;
        let error_ptr = *e_ptr;
        let error = if !error_ptr.is_null() {
            CStr::from_ptr(error_ptr).to_string_lossy()
        } else {
            std::borrow::Cow::Borrowed("")
        };

        println!(
            "##teamcity[testFailed name='{}: {}' message='Unhandled exception' details='{}']",
            test_suite, name, error
        );
    }

    let _ = std::io::stdout().flush();
}
