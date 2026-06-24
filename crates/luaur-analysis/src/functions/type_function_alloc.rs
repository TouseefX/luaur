pub unsafe extern "C" fn type_function_alloc(
    _ud: *mut core::ffi::c_void,
    ptr: *mut core::ffi::c_void,
    osize: usize,
    nsize: usize,
) -> *mut core::ffi::c_void {
    if nsize == 0 {
        if !ptr.is_null() {
            std::alloc::dealloc(
                ptr as *mut u8,
                std::alloc::Layout::from_size_align_unchecked(osize, 8),
            );
        }
        core::ptr::null_mut()
    } else if osize == 0 {
        std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(nsize, 8))
            as *mut core::ffi::c_void
    } else {
        let data = std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(nsize, 8))
            as *mut core::ffi::c_void;

        if !data.is_null() && !ptr.is_null() {
            let copy_size = if nsize < osize { nsize } else { osize };
            core::ptr::copy_nonoverlapping(ptr, data, copy_size);

            std::alloc::dealloc(
                ptr as *mut u8,
                std::alloc::Layout::from_size_align_unchecked(osize, 8),
            );
        }

        data
    }
}
