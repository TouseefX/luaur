use core::ffi::c_void;

pub unsafe extern "C" fn limited_realloc(
    _ud: *mut c_void,
    ptr: *mut c_void,
    osize: usize,
    nsize: usize,
) -> *mut c_void {
    let _ = osize;

    if nsize == 0 {
        unsafe {
            libc::free(ptr);
        }
        core::ptr::null_mut()
    } else if nsize > 8 * 1024 * 1024 {
        // For testing purposes return null for large allocations so we can generate errors related to memory allocation failures
        core::ptr::null_mut()
    } else {
        unsafe { libc::realloc(ptr, nsize) }
    }
}

mod libc {
    use core::ffi::c_void;

    extern "C" {
        pub fn free(ptr: *mut c_void);
        pub fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    }
}
