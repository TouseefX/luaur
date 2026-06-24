use core::ffi::c_char;
use core::ffi::CStr;

#[allow(non_snake_case)]
pub fn assertion_handler(
    expr: *const c_char,
    file: *const c_char,
    line: i32,
    _function: *const c_char,
) -> i32 {
    unsafe {
        let file_str = CStr::from_ptr(file).to_string_lossy();
        let expr_str = CStr::from_ptr(expr).to_string_lossy();

        println!("{}({}): ASSERTION FAILED: {}", file_str, line, expr_str);
    }
    1
}
