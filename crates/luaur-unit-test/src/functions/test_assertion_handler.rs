use core::ffi::c_char;

use crate::functions::debugger_present::debugger_present;

pub fn test_assertion_handler(
    expr: *const c_char,
    file: *const c_char,
    line: i32,
    function: *const c_char,
) -> i32 {
    if debugger_present() {
        return 1;
    }

    let expr_str = unsafe { core::ffi::CStr::from_ptr(expr).to_string_lossy() };
    let file_str = unsafe { core::ffi::CStr::from_ptr(file).to_string_lossy() };
    let function_str = unsafe { core::ffi::CStr::from_ptr(function).to_string_lossy() };

    panic!(
        "Assertion failed: {} at {}:{} in {}",
        expr_str, file_str, line, function_str
    );
}
