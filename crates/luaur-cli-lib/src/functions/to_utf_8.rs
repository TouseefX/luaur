use alloc::string::String;
use core::ffi::c_int;

use luaur_common::macros::luau_assert::LUAU_ASSERT;

#[allow(non_upper_case_globals)]
const CP_UTF8: u32 = 65001;

extern "system" {
    fn WideCharToMultiByte(
        code_page: u32,
        dw_flags: u32,
        lp_wide_char_str: *const u16,
        cch_wide_char: c_int,
        lp_multi_byte_str: *mut core::ffi::c_char,
        cb_multi_byte: c_int,
        lp_default_char: *const core::ffi::c_char,
        lp_used_default_char: *mut i32,
    ) -> c_int;
}

pub fn to_utf_8(path: &[u16]) -> String {
    let path_ptr = path.as_ptr();
    let path_len = path.len() as c_int;

    let result = unsafe {
        WideCharToMultiByte(
            CP_UTF8,
            0,
            path_ptr,
            path_len,
            core::ptr::null_mut(),
            0,
            core::ptr::null(),
            core::ptr::null_mut(),
        )
    };
    LUAU_ASSERT!(result > 0);

    let mut buf = vec![0u8; result as usize];
    unsafe {
        WideCharToMultiByte(
            CP_UTF8,
            0,
            path_ptr,
            path_len,
            buf.as_mut_ptr() as *mut core::ffi::c_char,
            result,
            core::ptr::null(),
            core::ptr::null_mut(),
        );
    }

    // Safety: WideCharToMultiByte with CP_UTF8 always produces valid UTF-8.
    unsafe { String::from_utf8_unchecked(buf) }
}
