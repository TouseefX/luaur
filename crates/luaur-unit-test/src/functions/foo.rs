pub fn foo(p: *const core::ffi::c_int) -> core::ffi::c_int {
    unsafe { *p }
}
