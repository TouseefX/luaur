use crate::records::non_exceptional_recursion_limiter::NonExceptionalRecursionLimiter;
use crate::records::recursion_counter::RecursionCounter;

impl NonExceptionalRecursionLimiter {
    pub fn non_exceptional_recursion_limiter_non_exceptional_recursion_limiter(
        &mut self,
        count: *mut core::ffi::c_int,
    ) {
        unsafe {
            // Initialize the base RecursionCounter with the provided count pointer
            // The base field is the first field, so we can write directly to it
            core::ptr::write(
                &mut self.base as *mut _ as *mut RecursionCounter,
                RecursionCounter::recursion_counter_i32(count),
            );
        }

        self.native_stack_guard.native_stack_guard();
    }
}
