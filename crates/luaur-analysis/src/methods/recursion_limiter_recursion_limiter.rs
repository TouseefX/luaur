use crate::records::internal_compiler_error::InternalCompilerError;
use crate::records::recursion_counter::RecursionCounter;
use crate::records::recursion_limit_exception::RecursionLimitException;
use crate::records::recursion_limiter::RecursionLimiter;
use alloc::format;
use alloc::string::String;

impl RecursionLimiter {
    pub fn recursion_limiter_recursion_limiter(
        &mut self,
        system: &str,
        count: *mut core::ffi::c_int,
        limit: core::ffi::c_int,
    ) {
        unsafe {
            core::ptr::write(
                &mut self.base as *mut RecursionCounter,
                RecursionCounter::recursion_counter_i32(count),
            );
        }

        self.native_stack_guard.native_stack_guard();

        if !self.native_stack_guard.is_ok() {
            let err = InternalCompilerError::internal_compiler_error_string(format!(
                "Stack overflow in {}",
                system
            ));
            std::panic::panic_any(err);
        }

        if limit > 0 && unsafe { *self.base.count > limit } {
            let err = RecursionLimitException::new(system);
            // Panic with the exception's owned message String. The previous
            // `CStr::from_ptr(err.base.what())` read the String's bytes as a
            // NUL-terminated C string and over-ran past the end, so the surfaced
            // message carried trailing garbage (e.g. "...in areEqual5\u{fffd}").
            // luaD_rawrunprotected downcasts this String payload into a LUA_ERRRUN
            // message, so a user type function that trips the limit reports
            // "'<fn>' type function errored at runtime: Internal recursion counter
            // limit exceeded in <system>" (type_function_user_udtf_areequal_*).
            panic!("{}", err.base.message);
        }
    }
}
