use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn place_unary_mod_reg_mem(
        &mut self,
        name: *const core::ffi::c_char,
        op: OperandX64,
        code8: u8,
        code: u8,
        opreg: u8,
    ) {
        if self.log_text {
            self.log_c_char_operand_x_64(name, op);
        }

        if !(op.cat == CategoryX64::reg || op.cat == CategoryX64::mem) {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        let size = if op.cat == CategoryX64::reg {
            op.base.size()
        } else {
            op.memSize
        };

        if !(size == SizeX64::byte || size == SizeX64::dword || size == SizeX64::qword) {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        self.place_rex_operand_x_64(op);
        self.place(if size == SizeX64::byte { code8 } else { code });
        self.place_mod_reg_mem(op, opreg, 0);

        self.commit();
    }
}
