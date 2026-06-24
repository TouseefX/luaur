use crate::records::boost_like_reporter::BoostLikeReporter;
use core::ffi::{c_char, c_void, CStr};
use std::io::Write;

pub fn boost_like_reporter_test_case_end(reporter: &mut BoostLikeReporter, _tc: *const c_void) {
    // Note: LUAU_ASSERT is provided by luaur_common, but since the crate dependency
    // resolution failed in the previous environment, we use the standard assert
    // to ensure compilation while maintaining the logic.
    assert!(!reporter.current_test.is_null());

    unsafe {
        // Based on the layout established in boost_like_reporter_test_case_start:
        // struct TestCase {
        //     const char* m_file;        // offset 0
        //     int m_line;                // offset 8
        //     const char* m_name;        // offset 16
        //     const char* m_test_suite;  // offset 24
        // };
        let name_ptr = *((reporter.current_test as *const u8).add(16) as *const *const c_char);
        let test_suite_ptr =
            *((reporter.current_test as *const u8).add(24) as *const *const c_char);

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

        println!("Leaving test case \"{}\"", name);
        println!("Leaving test suite \"{}\"", test_suite);
    }

    reporter.current_test = core::ptr::null();

    let _ = std::io::stdout().flush();
}
