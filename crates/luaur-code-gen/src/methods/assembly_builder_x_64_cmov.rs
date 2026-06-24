use crate::enums::category_x_64::CategoryX64;
use crate::enums::condition_x_64::ConditionX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl AssemblyBuilderX64 {
    pub fn cmov(&mut self, cond: ConditionX64, lhs: RegisterX64, rhs: OperandX64) {
        let size = if rhs.cat == CategoryX64::reg {
            rhs.base.size()
        } else {
            rhs.memSize
        };

        if !(size != SizeX64::byte && size == lhs.size()) {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        if self.log_text {
            let cond_idx = cond as usize;
            let cmov_text = [
                c"cmovo", c"cmovno", c"cmovc", c"cmovnc", c"cmovb", c"cmovbe", c"cmova", c"cmovae",
                c"cmove", c"cmovl", c"cmovle", c"cmovg", c"cmovge", c"cmovnb", c"cmovnbe",
                c"cmovna", c"cmovnae", c"cmovne", c"cmovnl", c"cmovnle", c"cmovng", c"cmovnge",
                c"cmovz", c"cmovnz", c"cmovp", c"cmovnp",
            ];
            self.log_c_char_operand_x_64_operand_x_64(
                cmov_text[cond_idx].as_ptr(),
                OperandX64::reg(lhs),
                rhs,
            );
        }

        self.place_rex_register_x_64_operand_x_64(lhs, rhs);
        self.place(0x0f);

        let code_for_condition = [
            0x0, 0x1, 0x2, 0x3, 0x2, 0x6, 0x7, 0x3, 0x4, 0xc, 0xe, 0xf, 0xd, 0x3, 0x7, 0x6, 0x2,
            0x5, 0xd, 0xf, 0xe, 0xc, 0x4, 0x5, 0xa, 0xb,
        ];
        self.place(0x40 | code_for_condition[cond as usize]);

        self.place_reg_and_mod_reg_mem(OperandX64::reg(lhs), rhs, 0);
        self.commit();
    }
}
