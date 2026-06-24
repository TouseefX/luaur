use crate::records::internal_compiler_error::InternalCompilerError;
use core::ffi::c_char;

impl InternalCompilerError {
    #[inline]
    pub fn what(&self) -> *const c_char {
        self.message.as_ptr() as *const c_char
    }
}
