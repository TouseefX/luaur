use core::ffi::c_char;

use crate::functions::set_luau_flag::set_luau_flag;
use crate::functions::set_luau_flags_flags::set_luau_flags_bool;

pub fn set_luau_flags_c_char(list: *const c_char) {
    if list.is_null() {
        return;
    }

    let list_str = unsafe { core::ffi::CStr::from_ptr(list).to_string_lossy() };
    let list_slice = list_str.as_bytes();

    let mut rest = list_slice;

    while !rest.is_empty() {
        let ending = rest.iter().position(|&b| b == b',');
        let element = if let Some(pos) = ending {
            &rest[..pos]
        } else {
            &rest[..]
        };

        if let Some(separator) = element.iter().position(|&b| b == b'=') {
            let key = &element[..separator];
            let value = &element[separator + 1..];

            let key_str = unsafe {
                core::ffi::CStr::from_ptr(key.as_ptr() as *const c_char).to_string_lossy()
            };
            let value_str = unsafe {
                core::ffi::CStr::from_ptr(value.as_ptr() as *const c_char).to_string_lossy()
            };

            if value_str == "true" || value_str == "True" {
                set_luau_flag(&key_str, true);
            } else if value_str == "false" || value_str == "False" {
                set_luau_flag(&key_str, false);
            } else {
                eprintln!("Warning: unrecognized value '{}'.", value_str);
            }
        } else {
            let element_str = unsafe {
                core::ffi::CStr::from_ptr(element.as_ptr() as *const c_char).to_string_lossy()
            };

            if element_str == "true" || element_str == "True" {
                set_luau_flags_bool(true);
            } else if element_str == "false" || element_str == "False" {
                set_luau_flags_bool(false);
            } else {
                set_luau_flag(&element_str, true);
            }
        }

        if let Some(pos) = ending {
            rest = &rest[pos + 1..];
        } else {
            break;
        }
    }
}
