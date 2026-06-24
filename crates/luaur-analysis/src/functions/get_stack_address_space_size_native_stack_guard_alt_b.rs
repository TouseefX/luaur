pub fn get_stack_address_space_size() -> usize {
    #[cfg(target_os = "macos")]
    {
        extern "C" {
            fn pthread_self() -> *mut core::ffi::c_void;
            fn pthread_get_stacksize_np(thread: *mut core::ffi::c_void) -> usize;
        }
        unsafe {
            let self_thread = pthread_self();
            pthread_get_stacksize_np(self_thread)
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        // The source provided specifically uses pthread_get_stacksize_np,
        // which is a non-portable Apple/BSD extension.
        0
    }
}
