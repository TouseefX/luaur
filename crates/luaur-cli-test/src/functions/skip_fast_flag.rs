use core::ffi::{c_char, c_int};

#[allow(non_snake_case)]
pub fn skip_fast_flag(flagName: *const c_char) -> bool {
    if flagName.is_null() {
        return false;
    }

    let flag_str = unsafe { core::ffi::CStr::from_ptr(flagName).to_bytes() };

    if flag_str.len() >= 4 && &flag_str[0..4] == b"Test" {
        return true;
    }

    if flag_str.len() >= 5 && &flag_str[0..5] == b"Debug" {
        return true;
    }

    false
}
