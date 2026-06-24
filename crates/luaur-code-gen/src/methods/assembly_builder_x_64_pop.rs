use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::op_plus_reg::OP_PLUS_REG;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn pop(&mut self, op: OperandX64) {
        if self.log_text {
            self.log_c_char_operand_x_64(c"pop".as_ptr(), op);
        }

        if !(op.cat == CategoryX64::reg && op.base.size() == SizeX64::qword) {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        self.place_rex_register_x_64(op.base);
        self.place(OP_PLUS_REG(0x58, op.base.index()));
        self.commit();
    }
}
