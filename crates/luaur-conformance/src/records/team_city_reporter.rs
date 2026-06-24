extern crate alloc;

use alloc::string::String;
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TeamCityReporter {
    pub(crate) current_test: *const c_void,
}

impl TeamCityReporter {
    pub fn team_city_reporter(&mut self) {}

    pub fn report_query(&mut self, _query: &c_void) {}

    pub fn test_run_start(&mut self) {}

    pub fn test_run_end(&mut self, _stats: &c_void) {}

    pub fn test_case_start(&mut self, in_data: &c_void) {
        self.current_test = in_data as *const c_void;

        unsafe {
            let in_test_suite = crate_external::get_doctest_field_str(in_data, "m_test_suite");
            let in_name = crate_external::get_doctest_field_str(in_data, "m_name");

            let _ = crate_external::c_printf_3_str(
                "##teamcity[testStarted name='%s: %s' captureStandardOutput='true']\n",
                &in_test_suite,
                &in_name,
                &String::new(),
            );
            let _ = crate_external::c_fflush_stdout();
        }
    }

    // called when a test case is reentered because of unfinished subcases
    pub fn test_case_reenter(&mut self, _in_data: &c_void) {}

    pub fn test_case_end(&mut self, in_data: &c_void) {
        unsafe {
            let current_test = self.current_test;
            let test_suite = crate_external::get_doctest_field_str(current_test, "m_test_suite");
            let name = crate_external::get_doctest_field_str(current_test, "m_name");

            let num_asserts_current_test =
                crate_external::get_doctest_field_i32(in_data, "numAssertsCurrentTest");
            let num_asserts_failed_current_test =
                crate_external::get_doctest_field_i32(in_data, "numAssertsFailedCurrentTest");
            let seconds = crate_external::get_doctest_field_f64(in_data, "seconds");

            let _ = crate_external::c_printf_3_i32(
                "##teamcity[testMetadata testName='%s: %s' name='total_asserts' type='number' value='%d']\n",
                &test_suite,
                &name,
                &num_asserts_current_test,
            );
            let _ = crate_external::c_printf_3_i32(
                "##teamcity[testMetadata testName='%s: %s' name='failed_asserts' type='number' value='%d']\n",
                &test_suite,
                &name,
                &num_asserts_failed_current_test,
            );
            let _ = crate_external::c_printf_3_f64(
                "##teamcity[testMetadata testName='%s: %s' name='runtime' type='number' value='%f']\n",
                &test_suite,
                &name,
                &seconds,
            );

            let test_case_success =
                crate_external::get_doctest_field_bool(in_data, "testCaseSuccess");
            if !test_case_success {
                let _ = crate_external::c_printf_2(
                    "##teamcity[testFailed name='%s: %s']\n",
                    &test_suite,
                    &name,
                );
            }

            let _ = crate_external::c_printf_2(
                "##teamcity[testFinished name='%s: %s']\n",
                &test_suite,
                &name,
            );
            let _ = crate_external::c_fflush_stdout();
        }
    }

    pub fn test_case_exception(&mut self, in_data: &c_void) {
        unsafe {
            let current_test = self.current_test;
            let test_suite = crate_external::get_doctest_field_str(current_test, "m_test_suite");
            let name = crate_external::get_doctest_field_str(current_test, "m_name");

            let error_string = crate_external::get_doctest_field_cstr(in_data, "error_string");

            let _ = crate_external::c_printf_3_str(
                "##teamcity[testFailed name='%s: %s' message='Unhandled exception' details='%s']\n",
                &test_suite,
                &name,
                &error_string,
            );
            let _ = crate_external::c_fflush_stdout();
        }
    }

    pub fn subcase_start(&mut self, _in_data: &c_void) {}

    pub fn subcase_end(&mut self) {}

    pub fn log_assert(&mut self, _ad: &c_void) {}

    pub fn log_message(&mut self, _md: &c_void) {}

    pub fn test_case_skipped(&mut self, in_data: &c_void) {
        unsafe {
            let test_suite = crate_external::get_doctest_field_str(in_data, "m_test_suite");
            let name = crate_external::get_doctest_field_str(in_data, "m_name");

            let _ = crate_external::c_printf_2(
                "##teamcity[testIgnored name='%s: %s' captureStandardOutput='false']\n",
                &test_suite,
                &name,
            );
        }
    }
}

// The following helper functions are expected to exist in this crate (generated or hand-ported).
// They provide field access for doctest structs and stdio printing.
mod crate_external {
    use super::*;

    extern "Rust" {
        pub(crate) fn get_doctest_field_str(base: *const c_void, field: &str) -> String;
        pub(crate) fn get_doctest_field_i32(base: *const c_void, field: &str) -> i32;
        pub(crate) fn get_doctest_field_f64(base: *const c_void, field: &str) -> f64;
        pub(crate) fn get_doctest_field_bool(base: *const c_void, field: &str) -> bool;
        pub(crate) fn get_doctest_field_cstr(base: *const c_void, field: &str) -> String;

        // Each signature must have a unique Rust name; overloading `c_printf` is not supported.
        pub(crate) fn c_printf_2(fmt: &str, a0: &String, a1: &String) -> i32;
        pub(crate) fn c_printf_3_i32(fmt: &str, a0: &String, a1: &String, a2: &i32) -> i32;
        pub(crate) fn c_printf_3_f64(fmt: &str, a0: &String, a1: &String, a2: &f64) -> i32;
        pub(crate) fn c_printf_3_str(fmt: &str, a0: &String, a1: &String, a2: &String) -> i32;

        pub(crate) fn c_fflush_stdout() -> i32;
    }
}
