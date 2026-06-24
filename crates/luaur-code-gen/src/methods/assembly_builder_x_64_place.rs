use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
    pub fn place(&mut self, byte: u8) {
        if self.code_pos >= self.code_end {
            unsafe {
                luaur_common::assert_call_handler(
                    b"codePos < codeEnd\0".as_ptr() as *const core::ffi::c_char,
                    b"CodeGen/src/AssemblyBuilderX64.cpp\0".as_ptr() as *const core::ffi::c_char,
                    1748,
                    b"void Luau::CodeGen::AssemblyBuilderX64::place(uint8_t)\0".as_ptr()
                        as *const core::ffi::c_char,
                );
                luaur_common::LUAU_DEBUGBREAK!();
            }
        }
        unsafe {
            *self.code_pos = byte;
            self.code_pos = self.code_pos.add(1);
        }
    }
}
