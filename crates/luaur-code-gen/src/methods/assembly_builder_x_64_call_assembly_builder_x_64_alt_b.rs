use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn call_operand_x_64(&mut self, op: OperandX64) {
        let check = (if op.cat == CategoryX64::reg {
            op.base.size()
        } else {
            op.memSize
        }) == SizeX64::qword;
        if !check {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        if self.log_text {
            self.log_c_char_operand_x_64(c"call".as_ptr(), op);
        }

        // Indirect absolute calls always work in 64 bit width mode, so REX.W is optional
        self.place_rex_no_w(op);

        self.place(0xff);
        self.place_mod_reg_mem(op, 2, 0);
        self.commit();
    }
}
