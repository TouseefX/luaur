use core::ffi::c_void;
use std::alloc::{alloc, dealloc, realloc, Layout};

const ALIGNMENT: usize = 16;

pub unsafe extern "C" fn userdata_alignment_alloc(
    _ud: *mut c_void,
    ptr: *mut c_void,
    osize: usize,
    nsize: usize,
) -> *mut c_void {
    if nsize == 0 {
        if !ptr.is_null() && osize != 0 {
            let layout = Layout::from_size_align(osize, ALIGNMENT).expect("valid old layout");
            dealloc(ptr.cast(), layout);
        }

        return core::ptr::null_mut();
    }

    let new_layout = Layout::from_size_align(nsize, ALIGNMENT).expect("valid new layout");
    if ptr.is_null() || osize == 0 {
        alloc(new_layout).cast()
    } else {
        let old_layout = Layout::from_size_align(osize, ALIGNMENT).expect("valid old layout");
        realloc(ptr.cast(), old_layout, nsize).cast()
    }
}
