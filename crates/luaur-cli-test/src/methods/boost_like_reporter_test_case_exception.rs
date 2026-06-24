use crate::records::boost_like_reporter::BoostLikeReporter;
use core::ffi::{c_char, c_void, CStr};
use std::io::Write;

// EXTERNAL_CRATE_REQUIRED: doctest - used for TestCaseException type
// The doctest::TestCaseException type is not translated; we treat it as an opaque pointer
// or assume the C++ ABI layout for accessing fields.

pub fn boost_like_reporter_test_case_exception(reporter: &BoostLikeReporter, e: *const c_void) {
    // LUAU_ASSERT is not available here due to crate resolution issues; using standard assert.
    assert!(!reporter.current_test.is_null());

    unsafe {
        // doctest::TestCaseException layout (approximate):
        // struct TestCaseException {
        //     std::string error_string;
        //     bool is_crash;
        // };
        // We assume the error_string is at offset 0 and is a std::string (pointer to char buffer).
        let e_ptr = e as *const *const c_char;
        let error_ptr = *e_ptr;
        let error = CStr::from_ptr(error_ptr).to_string_lossy();

        // TestCase pointer layout (from previous context):
        // struct TestCase {
        //     const char* m_file;
        //     int m_line;
        //     ...
        // };
        let tc_ptr = reporter.current_test as *const *const c_char;
        let file_ptr = *tc_ptr;
        let line_ptr = (reporter.current_test as *const u8).add(8) as *const i32;
        let line = *line_ptr;
        let file = CStr::from_ptr(file_ptr).to_string_lossy();

        println!("{}({}): FATAL: Unhandled exception {}", file, line, error);
    }

    let _ = std::io::stdout().flush();
}
