use crate::records::runtime_navigation_context::RuntimeNavigationContext;
use core::ffi::{c_char, c_void};

impl RuntimeNavigationContext {
    pub fn get_cache_key(&self) -> Option<alloc::string::String> {
        let config = unsafe { &*self.config };
        if let Some(writer) = config.get_cache_key {
            let initial_buffer_size = 256;
            let safe_writer = unsafe {
                core::mem::transmute::<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_char, usize, *mut usize) -> crate::enums::luarequire_write_result::luarequire_WriteResult, extern "C" fn(*mut c_void, *mut c_void, *mut c_char, usize, *mut usize) -> crate::enums::luarequire_write_result::luarequire_WriteResult>(writer)
            };
            self.get_string_from_c_writer(safe_writer, initial_buffer_size)
        } else {
            None
        }
    }
}
