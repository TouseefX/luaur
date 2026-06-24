use core::ffi::c_void;
use core::sync::atomic::{AtomicUsize, Ordering};

pub static HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_TO_FAIL: AtomicUsize = AtomicUsize::new(0);
pub static HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);

extern "C" {
    fn free(ptr: *mut c_void);
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

pub unsafe extern "C" fn huge_function_load_failure_test_allocate(
    _ud: *mut c_void,
    ptr: *mut c_void,
    _osize: usize,
    nsize: usize,
) -> *mut c_void {
    if nsize == 0 {
        free(ptr);
        core::ptr::null_mut()
    } else if nsize > 32768 {
        if HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_COUNT.load(Ordering::SeqCst)
            == HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_TO_FAIL.load(Ordering::SeqCst)
        {
            core::ptr::null_mut()
        } else {
            HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_COUNT.fetch_add(1, Ordering::SeqCst);
            realloc(ptr, nsize)
        }
    } else {
        realloc(ptr, nsize)
    }
}
