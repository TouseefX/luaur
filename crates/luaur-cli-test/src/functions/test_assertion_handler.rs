use core::ffi::{c_char, c_int};

use crate::functions::debugger_present::debugger_present;

#[allow(non_snake_case)]
pub fn test_assertion_handler(
    expr: *const c_char,
    file: *const c_char,
    line: c_int,
    function: *const c_char,
) -> c_int {
    let _function = function;

    if debugger_present() {
        return 1;
    }

    let expr_str = if expr.is_null() {
        "<unknown>".to_string()
    } else {
        unsafe { core::ffi::CStr::from_ptr(expr) }
            .to_string_lossy()
            .into_owned()
    };

    let file_str = if file.is_null() {
        "<unknown>".to_string()
    } else {
        unsafe { core::ffi::CStr::from_ptr(file) }
            .to_string_lossy()
            .into_owned()
    };

    let _ = format!("Assertion failed: {} ({}:{})", expr_str, file_str, line);

    1
}
