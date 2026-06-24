use crate::enums::luarequire_write_result::luarequire_WriteResult;
use crate::records::runtime_navigation_context::RuntimeNavigationContext;
use core::ffi::{c_char, c_void};

impl RuntimeNavigationContext {
    pub fn get_loadname(&self) -> Option<alloc::string::String> {
        unsafe {
            let config = &*self.config;
            let writer = config.get_loadname?;
            let initial_identifier_buffer_size = 256;

            let safe_writer: extern "C" fn(
                *mut c_void,
                *mut c_void,
                *mut c_char,
                usize,
                *mut usize,
            ) -> luarequire_WriteResult = core::mem::transmute(writer);

            self.get_string_from_c_writer(safe_writer, initial_identifier_buffer_size)
        }
    }
}
