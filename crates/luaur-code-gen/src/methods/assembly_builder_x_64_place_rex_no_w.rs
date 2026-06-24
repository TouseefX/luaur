use crate::enums::category_x_64::CategoryX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::rex_b::REX_B;
use crate::macros::rex_x::REX_X;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn place_rex_no_w(&mut self, op: OperandX64) {
        let mut code: u8 = 0;

        if op.cat == CategoryX64::reg {
            code = REX_B(op.base);
        } else if op.cat == CategoryX64::mem {
            code = REX_X(op.index) | REX_B(op.base);
        } else {
            // Avoid CODEGEN_ASSERT! due to assert_call_handler signature mismatch in this crate.
            luaur_common::LUAU_DEBUGBREAK!();
        }

        if code != 0 {
            self.place(code | 0x40);
        }
    }
}
