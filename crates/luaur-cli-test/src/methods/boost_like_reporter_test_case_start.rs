use crate::records::boost_like_reporter::BoostLikeReporter;
use core::ffi::{c_char, c_void, CStr};
use std::io::Write;

pub fn boost_like_reporter_test_case_start(reporter: &mut BoostLikeReporter, tc: *const c_void) {
    reporter.current_test = tc;

    unsafe {
        let name_ptr = *((tc as *const u8).add(16) as *const *const c_char);
        let test_suite_ptr = *((tc as *const u8).add(24) as *const *const c_char);

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

        println!("Entering test suite \"{}\"", test_suite);
        println!("Entering test case \"{}\"", name);
    }

    let _ = std::io::stdout().flush();
}
