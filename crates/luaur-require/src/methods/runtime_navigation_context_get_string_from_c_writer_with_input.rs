use crate::enums::luarequire_write_result::luarequire_WriteResult;
use crate::records::runtime_navigation_context::RuntimeNavigationContext;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::{c_char, c_void};

impl RuntimeNavigationContext {
    pub fn get_string_from_c_writer_with_input(
        &self,
        writer: extern "C" fn(
            *mut c_void,
            *mut c_void,
            *const c_char,
            *mut c_char,
            usize,
            *mut usize,
        ) -> luarequire_WriteResult,
        input: String,
        initial_buffer_size: usize,
    ) -> Option<String> {
        let mut buffer = Vec::new();
        buffer.resize(initial_buffer_size, 0);

        let mut size: usize = 0;
        let c_input = alloc::ffi::CString::new(input).ok()?;
        let result = unsafe {
            writer(
                self.l,
                self.ctx,
                c_input.as_ptr(),
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len(),
                &mut size,
            )
        };

        let final_result = if result == luarequire_WriteResult::WRITE_BUFFER_TOO_SMALL {
            buffer.resize(size, 0);
            unsafe {
                writer(
                    self.l,
                    self.ctx,
                    c_input.as_ptr(),
                    buffer.as_mut_ptr() as *mut c_char,
                    buffer.len(),
                    &mut size,
                )
            }
        } else {
            result
        };

        if final_result == luarequire_WriteResult::WRITE_SUCCESS {
            // `size` is the null-terminated size; exclude the trailing NUL.
            buffer.truncate(size.saturating_sub(1));
            return String::from_utf8(buffer).ok();
        }

        None
    }
}
