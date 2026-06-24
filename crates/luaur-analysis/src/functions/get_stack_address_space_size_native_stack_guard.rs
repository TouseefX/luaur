#[allow(non_snake_case)]
#[cfg(target_os = "windows")]
extern "system" {
    fn GetCurrentThreadStackLimits(low_limit: *mut usize, high_limit: *mut usize);
}

pub fn get_stack_address_space_size() -> usize {
    #[cfg(target_os = "windows")]
    {
        let mut low: usize = 0;
        let mut high: usize = 0;
        unsafe {
            GetCurrentThreadStackLimits(&mut low, &mut high);
        }
        high.wrapping_sub(low)
    }

    #[cfg(not(target_os = "windows"))]
    {
        // On non-Windows platforms, this function is typically not used or
        // would require platform-specific pthread_attr_getstack calls.
        // The Luau source provided specifically uses the Windows API.
        0
    }
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use get_stack_address_space_size as get_stack_address_space_size_mut_2;
