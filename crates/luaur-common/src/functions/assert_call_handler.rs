use crate::functions::assert_handler::assert_handler;
use crate::macros::luau_noinline::LUAU_NOINLINE;
use core::ffi::c_char;

LUAU_NOINLINE! {
    pub fn assert_call_handler(
        expression: *const c_char,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> i32 {
        let handler_ptr = assert_handler();
        if let Some(handler) = *handler_ptr {
            unsafe {
                return handler(expression, file, line, function);
            }
        }

        1
    }
}
