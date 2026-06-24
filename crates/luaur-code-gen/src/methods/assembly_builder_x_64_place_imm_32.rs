use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
    pub fn place_imm_32(&mut self, imm: i32) {
        let pos = self.code_pos;
        if !((pos as usize).wrapping_add(core::mem::size_of_val(&imm)) < self.code_end as usize) {
            luaur_common::LUAU_DEBUGBREAK!();
        }
        unsafe {
            core::ptr::copy_nonoverlapping(
                imm.to_le_bytes().as_ptr(),
                pos,
                core::mem::size_of_val(&imm),
            );
            self.code_pos = pos.add(core::mem::size_of_val(&imm));
        }
    }
}
