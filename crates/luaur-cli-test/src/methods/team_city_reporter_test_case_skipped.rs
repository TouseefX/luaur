use crate::records::team_city_reporter::TeamCityReporter;
use core::ffi::{c_char, c_void, CStr};

pub fn team_city_reporter_test_case_skipped(_self: &mut TeamCityReporter, in_: *const c_void) {
    unsafe {
        let in_bytes = in_ as *const u8;

        let test_suite_ptr = *((in_bytes.add(0x10)) as *const *const c_char);
        let name_ptr = *((in_bytes.add(0x18)) as *const *const c_char);

        let test_suite = if !test_suite_ptr.is_null() {
            CStr::from_ptr(test_suite_ptr).to_string_lossy()
        } else {
            "".into()
        };

        let name = if !name_ptr.is_null() {
            CStr::from_ptr(name_ptr).to_string_lossy()
        } else {
            "".into()
        };

        println!(
            "##teamcity[testIgnored name='{}: {}' captureStandardOutput='false']",
            test_suite, name
        );
    }
}
