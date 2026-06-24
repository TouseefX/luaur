use crate::records::compile_error::CompileError;
use core::ffi::c_char;

impl CompileError {
    pub fn what(&self) -> *const c_char {
        self.message.as_ptr() as *const c_char
    }
}
