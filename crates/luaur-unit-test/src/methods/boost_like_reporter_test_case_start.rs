use luaur_common::macros::luau_assert::LUAU_ASSERT;
use std::io::Write;

impl crate::records::boost_like_reporter::BoostLikeReporter {
    pub fn test_case_start(
        &mut self,
        tc_test_suite: &str,
        tc_name: &str,
        tc_file: &str,
        tc_line: i32,
    ) {
        // In the C++ source, currentTest is assigned the address of the TestCaseData object.
        // Since the Rust signature receives the fields decomposed, we use the file string
        // pointer as a stable identity for the current test context, matching the skeleton's
        // approach to satisfy the pointer type of the field.
        self.current_test = tc_file.as_ptr() as *const core::ffi::c_void;

        LUAU_ASSERT!(!self.current_test.is_null());

        println!("Entering test suite \"{}\"", tc_test_suite);
        println!("Entering test case \"{}\"", tc_name);
        let _ = std::io::stdout().flush();
        let _ = tc_line;
    }
}
