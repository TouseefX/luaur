use crate::records::runtime_navigation_context::RuntimeNavigationContext;

impl RuntimeNavigationContext {
    pub fn get_alias(&self, alias: &str) -> Option<alloc::string::String> {
        let input = alias.to_owned();
        let writer = unsafe { (*self.config).get_alias }?;
        let safe_writer: extern "C" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            *const core::ffi::c_char,
            *mut core::ffi::c_char,
            usize,
            *mut usize,
        ) -> crate::enums::luarequire_write_result::luarequire_WriteResult =
            unsafe { core::mem::transmute(writer) };
        self.get_string_from_c_writer_with_input(safe_writer, input, 256)
    }
}
