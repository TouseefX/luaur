use alloc::vec::Vec;

// `fromUtf8` lives inside `#ifdef _WIN32` in FileUtils.cpp — it converts a UTF-8
// path to the UTF-16 `wstring` the Win32 wide-char APIs require. The
// `MultiByteToWideChar` symbol only exists on Windows, so the implementation is
// gated; on POSIX no caller reaches it (all callers are themselves Win32-only).

#[cfg(windows)]
pub fn from_utf_8(path: &str) -> Vec<u16> {
    use core::ffi::c_int;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    #[allow(non_upper_case_globals)]
    const CP_UTF8: u32 = 65001;

    extern "system" {
        fn MultiByteToWideChar(
            code_page: u32,
            dw_flags: u32,
            lp_multi_byte_str: *const core::ffi::c_char,
            cb_multi_byte: c_int,
            lp_wide_char_str: *mut u16,
            cch_wide_char: c_int,
        ) -> c_int;
    }

    let path_bytes = path.as_bytes();
    let path_ptr = path_bytes.as_ptr() as *const core::ffi::c_char;
    let path_len = path_bytes.len() as c_int;

    let result =
        unsafe { MultiByteToWideChar(CP_UTF8, 0, path_ptr, path_len, core::ptr::null_mut(), 0) };
    LUAU_ASSERT!(result > 0);

    let mut buf = vec![0u16; result as usize];
    unsafe {
        MultiByteToWideChar(CP_UTF8, 0, path_ptr, path_len, buf.as_mut_ptr(), result);
    }

    buf
}

#[cfg(not(windows))]
pub fn from_utf_8(path: &str) -> Vec<u16> {
    // POSIX builds never invoke the Win32 wide-path conversion.
    let _ = path;
    Vec::new()
}
