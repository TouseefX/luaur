use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::op_plus_reg::OP_PLUS_REG;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl AssemblyBuilderX64 {
    pub fn bswap(&mut self, dst: RegisterX64) {
        if self.log_text {
            self.log_c_char_operand_x_64(c"bswap".as_ptr(), OperandX64::reg(dst));
        }

        if !(dst.size() == crate::enums::size_x_64::SizeX64::dword
            || dst.size() == crate::enums::size_x_64::SizeX64::qword)
        {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        self.place_rex_register_x_64(dst);
        self.place(0x0f);
        self.place(OP_PLUS_REG(0xc8, dst.index()));
        self.commit();
    }
}
