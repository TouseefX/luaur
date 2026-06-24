use core::ffi::c_char;

pub unsafe fn is_method_or_function_char(s: *const c_char, len: i64) -> bool {
    if len != 1 || s.is_null() {
        return false;
    }

    let c = *s as u8 as char;
    c.is_ascii_alphanumeric() || c == '.' || c == ':' || c == '_'
}
