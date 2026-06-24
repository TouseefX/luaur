pub unsafe extern "C" fn l_alloc(
    ud: *mut core::ffi::c_void,
    ptr: *mut core::ffi::c_void,
    osize: usize,
    nsize: usize,
) -> *mut core::ffi::c_void {
    let _ = ud;
    let _ = osize;

    unsafe {
        if nsize == 0 {
            let _ = ptr;
            if !ptr.is_null() {
                let _ = libc_free(ptr);
            }
            core::ptr::null_mut()
        } else {
            if ptr.is_null() {
                libc_realloc(core::ptr::null_mut(), nsize)
            } else {
                libc_realloc(ptr, nsize)
            }
        }
    }
}

unsafe fn libc_free(ptr: *mut core::ffi::c_void) {
    // wasm32-unknown-unknown generally does not support the full libc surface.
    // These are only called when the runtime provides an allocator-compatible backing.
    #[cfg(target_arch = "wasm32")]
    {
        // wasm32-unknown-unknown does not provide the libc allocator surface, and
        // the wasm `libc_realloc` path below never hands out real allocations, so
        // there is nothing to free here.
        let _ = ptr;
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        extern "C" {
            fn free(ptr: *mut core::ffi::c_void);
        }
        free(ptr);
    }
}

unsafe fn libc_realloc(ptr: *mut core::ffi::c_void, nsize: usize) -> *mut core::ffi::c_void {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = ptr;
        let _ = nsize;
        core::ptr::null_mut()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        extern "C" {
            fn realloc(ptr: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
        }
        realloc(ptr, nsize)
    }
}
