use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
    pub fn place(&mut self, word: u32) {
        if !(self.code_pos < self.code_end) {
            unsafe {
                luaur_common::assert_call_handler(
                    b"codePos < codeEnd\0".as_ptr() as *const core::ffi::c_char,
                    b"CodeGen/src/AssemblyBuilderA64.cpp\0".as_ptr() as *const core::ffi::c_char,
                    0,
                    b"void Luau::CodeGen::AssemblyBuilderA64::place(uint32_t)\0".as_ptr()
                        as *const core::ffi::c_char,
                );
                luaur_common::LUAU_DEBUGBREAK!();
            }
        }
        unsafe {
            *self.code_pos = word;
            self.code_pos = self.code_pos.add(1);
        }
    }
}
