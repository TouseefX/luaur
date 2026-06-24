use crate::records::non_exceptional_recursion_limiter::NonExceptionalRecursionLimiter;

impl NonExceptionalRecursionLimiter {
    #[inline]
    pub fn is_ok(&self, limit: core::ffi::c_int) -> bool {
        self.native_stack_guard.is_ok() && !(limit > 0 && unsafe { *self.base.count > limit })
    }
}
