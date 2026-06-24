#[allow(non_snake_case)]
pub unsafe fn getCwd(
    buffer: *mut core::ffi::c_char,
    maxlen: core::ffi::c_int,
) -> *mut core::ffi::c_char {
    extern "C" {
        #[cfg(windows)]
        #[link_name = "_getcwd"]
        fn getcwd_impl(
            buffer: *mut core::ffi::c_char,
            maxlen: core::ffi::c_int,
        ) -> *mut core::ffi::c_char;

        #[cfg(not(windows))]
        #[link_name = "getcwd"]
        fn getcwd_impl(
            buffer: *mut core::ffi::c_char,
            maxlen: core::ffi::c_int,
        ) -> *mut core::ffi::c_char;
    }

    getcwd_impl(buffer, maxlen)
}
