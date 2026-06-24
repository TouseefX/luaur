extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
use core::ffi::c_char;
use core::ffi::CStr;

pub(crate) fn wrong_number_of_args_string(
    expected_count: usize,
    maximum_count: Option<usize>,
    actual_count: usize,
    arg_prefix: *const c_char,
    is_variadic: bool,
) -> String {
    let mut s = String::from("expects ");

    if is_variadic {
        s += "at least ";
    }

    s += &expected_count.to_string();
    s += " ";

    if let Some(max) = maximum_count {
        if expected_count != max {
            s += "to ";
            s += &max.to_string();
            s += " ";
        }
    }

    if !arg_prefix.is_null() {
        s += &unsafe { CStr::from_ptr(arg_prefix).to_string_lossy() };
        s += " ";
    }

    s += "argument";
    if maximum_count.unwrap_or(expected_count) != 1 {
        s += "s";
    }

    s += ", but ";

    if actual_count == 0 {
        s += "none";
    } else {
        if actual_count < expected_count {
            s += "only ";
        }

        s += &actual_count.to_string();
    }

    s += if actual_count == 1 { " is" } else { " are" };

    s += " specified";

    s
}
