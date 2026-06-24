use crate::enums::luarequire_write_result::luarequire_WriteResult;
use crate::records::runtime_navigation_context::RuntimeNavigationContext;
use core::ffi::{c_char, c_void};

impl RuntimeNavigationContext {
    pub fn get_string_from_c_writer(
        &self,
        writer: extern "C" fn(
            *mut c_void,
            *mut c_void,
            *mut c_char,
            usize,
            *mut usize,
        ) -> luarequire_WriteResult,
        initial_buffer_size: usize,
    ) -> Option<alloc::string::String> {
        let mut buffer = alloc::vec::Vec::new();
        buffer.resize(initial_buffer_size, 0);

        let mut size: usize = 0;
        let result = unsafe {
            writer(
                self.l as *mut c_void,
                self.ctx as *mut c_void,
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len(),
                &mut size,
            )
        };

        if result == luarequire_WriteResult::WRITE_BUFFER_TOO_SMALL {
            buffer.resize(size, 0);
            let mut size2: usize = 0;
            let result2 = unsafe {
                writer(
                    self.l as *mut c_void,
                    self.ctx as *mut c_void,
                    buffer.as_mut_ptr() as *mut c_char,
                    buffer.len(),
                    &mut size2,
                )
            };
            if result2 == luarequire_WriteResult::WRITE_SUCCESS {
                // `size` is the null-terminated size; exclude the trailing NUL.
                buffer.truncate(size2.saturating_sub(1));
                return alloc::string::String::from_utf8(buffer).ok();
            }
        } else if result == luarequire_WriteResult::WRITE_SUCCESS {
            // `size` is the null-terminated size; exclude the trailing NUL.
            buffer.truncate(size.saturating_sub(1));
            return alloc::string::String::from_utf8(buffer).ok();
        }

        None
    }
}
