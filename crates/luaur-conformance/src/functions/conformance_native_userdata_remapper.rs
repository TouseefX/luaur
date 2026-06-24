use core::ffi::{c_char, c_void};

use crate::functions::compare_member_name::compare_member_name;

pub unsafe extern "C" fn conformance_native_userdata_remapper(
    _context: *mut c_void,
    name: *const c_char,
    name_length: usize,
) -> u8 {
    if compare_member_name(name, name_length, c"extra".as_ptr()) {
        return 0;
    }
    if compare_member_name(name, name_length, c"color".as_ptr()) {
        return 1;
    }
    if compare_member_name(name, name_length, c"vec2".as_ptr()) {
        return 2;
    }
    if compare_member_name(name, name_length, c"mat3".as_ptr()) {
        return 3;
    }
    if compare_member_name(name, name_length, c"vertex".as_ptr()) {
        return 4;
    }

    0xff
}
