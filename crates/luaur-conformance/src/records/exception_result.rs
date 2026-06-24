extern crate alloc;

use alloc::string::String;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct ExceptionResult {
    pub exception_generated: bool,
    pub description: String,
}
