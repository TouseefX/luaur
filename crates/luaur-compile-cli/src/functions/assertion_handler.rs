use core::ffi::{c_char, c_int, CStr};

#[allow(non_snake_case)]
pub fn assertion_handler(
    expr: *const c_char,
    file: *const c_char,
    line: c_int,
    _function: *const c_char,
) -> c_int {
    unsafe {
        let file_str = CStr::from_ptr(file).to_string_lossy();
        let expr_str = CStr::from_ptr(expr).to_string_lossy();

        // Use standard Rust printing to stdout to avoid dependency on the libc crate,
        // matching the behavior of the C++ printf call.
        std::print!("{}({}): ASSERTION FAILED: {}\n", file_str, line, expr_str);
    }
    1
}
