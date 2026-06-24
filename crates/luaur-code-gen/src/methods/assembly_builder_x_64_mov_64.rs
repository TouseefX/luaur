use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::op_plus_reg::OP_PLUS_REG;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::register_x_64::RegisterX64;

impl AssemblyBuilderX64 {
    pub fn mov64(&mut self, lhs: RegisterX64, imm: i64) {
        if self.log_text {
            self.text.push_str(" mov         ");
            self.log_operand_x_64(lhs.into());
            self.log_append(format_args!(",{:X}h\n", imm as u64));
        }

        if !(lhs.size() == SizeX64::qword) {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        self.place_rex_register_x_64(lhs);
        self.place(OP_PLUS_REG(0xb8, lhs.index()));
        self.place_imm_64(imm);
        self.commit();
    }
}
