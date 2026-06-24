use crate::functions::writeu_16::writeu_16;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
    pub fn place_imm_16(&mut self, imm: i16) {
        let pos = self.code_pos;
        unsafe {
            CODEGEN_ASSERT!(pos.add(core::mem::size_of::<i16>()) < self.code_end);
            self.code_pos = writeu_16(pos, imm as u16);
        }
    }
}
