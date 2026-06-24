use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::rex_b::REX_B;
use crate::macros::rex_force::REX_FORCE;
use crate::macros::rex_w_bit::REX_W_BIT;
use crate::macros::rex_x::REX_X;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn place_rex_operand_x_64(&mut self, op: OperandX64) {
        let mut code: u8 = 0;

        if op.cat == CategoryX64::reg {
            code =
                REX_W_BIT!(op.base.size() == SizeX64::qword) | REX_B(op.base) | REX_FORCE(op.base);
        } else if op.cat == CategoryX64::mem {
            code = REX_W_BIT!(op.memSize == SizeX64::qword) | REX_X(op.index) | REX_B(op.base);
        } else {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        if code != 0 {
            self.place(code | 0x40);
        }
    }
}
