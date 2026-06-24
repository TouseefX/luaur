use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl AssemblyBuilderX64 {
    pub fn movzx(&mut self, lhs: RegisterX64, rhs: OperandX64) {
        if self.log_text {
            self.log_c_char_operand_x_64_operand_x_64(c"movzx".as_ptr(), OperandX64::reg(lhs), rhs);
        }

        let size = if rhs.cat == CategoryX64::reg {
            rhs.base.size()
        } else {
            rhs.memSize
        };

        if !(size == SizeX64::byte || size == SizeX64::word) {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        self.place_rex_register_x_64_operand_x_64(lhs, rhs);
        self.place(0x0f);
        self.place(if size == SizeX64::byte { 0xb6 } else { 0xb7 });
        self.place_reg_and_mod_reg_mem(OperandX64::reg(lhs), rhs, 0);
        self.commit();
    }
}
