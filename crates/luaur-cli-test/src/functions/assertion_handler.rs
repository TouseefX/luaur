use core::ffi::{c_char, c_int};

#[allow(non_snake_case)]
pub fn assertion_handler(
    expr: *const c_char,
    file: *const c_char,
    line: c_int,
    _function: *const c_char,
) -> c_int {
    let expr_str = if expr.is_null() {
        "<unknown>".to_string()
    } else {
        unsafe {
            core::ffi::CStr::from_ptr(expr)
                .to_string_lossy()
                .into_owned()
        }
    };

    let file_str = if file.is_null() {
        "<unknown>".to_string()
    } else {
        unsafe {
            core::ffi::CStr::from_ptr(file)
                .to_string_lossy()
                .into_owned()
        }
    };

    println!("{}({}): ASSERTION FAILED: {}", file_str, line, expr_str);
    1
}
