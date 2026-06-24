use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::rex_b::REX_B;
use crate::macros::rex_force::REX_FORCE;
use crate::macros::rex_r::REX_R;
use crate::macros::rex_w_bit::REX_W_BIT;
use crate::macros::rex_x::REX_X;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl AssemblyBuilderX64 {
    pub fn place_rex_register_x_64_operand_x_64(&mut self, lhs: RegisterX64, rhs: OperandX64) {
        let mut code = REX_W_BIT!(lhs.size() == SizeX64::qword) | REX_FORCE(lhs);

        if rhs.cat == CategoryX64::imm {
            code |= REX_B(lhs);
        } else {
            if !(rhs.cat == CategoryX64::reg || rhs.cat == CategoryX64::mem) {
                luaur_common::LUAU_DEBUGBREAK!();
            }
            code |= REX_R(lhs)
                | REX_X(rhs.index)
                | REX_B(rhs.base)
                | REX_FORCE(lhs)
                | REX_FORCE(rhs.base);
        }

        if code != 0 {
            self.place(code | 0x40);
        }
    }
}
