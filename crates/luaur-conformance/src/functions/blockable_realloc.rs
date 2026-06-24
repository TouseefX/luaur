use crate::functions::blockable_realloc_allowed::blockableReallocAllowed;

pub unsafe extern "C" fn blockable_realloc(
    _ud: *mut core::ffi::c_void,
    ptr: *mut core::ffi::c_void,
    _osize: usize,
    nsize: usize,
) -> *mut core::ffi::c_void {
    if nsize == 0 {
        libc::free(ptr);
        core::ptr::null_mut()
    } else {
        if !blockableReallocAllowed {
            return core::ptr::null_mut();
        }
        libc::realloc(ptr, nsize)
    }
}

mod libc {
    use core::ffi::c_void;

    extern "C" {
        pub fn free(ptr: *mut c_void);
        pub fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    }
}
