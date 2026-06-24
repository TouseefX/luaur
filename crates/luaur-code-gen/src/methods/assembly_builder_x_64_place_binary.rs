use crate::enums::category_x_64::CategoryX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn place_binary(
        &mut self,
        name: *const core::ffi::c_char,
        lhs: OperandX64,
        rhs: OperandX64,
        codeimm8: u8,
        codeimm: u8,
        codeimm_imm8: u8,
        code8rev: u8,
        coderev: u8,
        code8: u8,
        code: u8,
        opreg: u8,
    ) {
        if self.log_text {
            self.log_c_char_operand_x_64_operand_x_64(name, lhs, rhs);
        }

        if (lhs.cat == CategoryX64::reg || lhs.cat == CategoryX64::mem)
            && rhs.cat == CategoryX64::imm
        {
            self.place_binary_reg_mem_and_imm(lhs, rhs, codeimm8, codeimm, codeimm_imm8, opreg);
        } else if lhs.cat == CategoryX64::reg
            && (rhs.cat == CategoryX64::reg || rhs.cat == CategoryX64::mem)
        {
            self.place_binary_reg_and_reg_mem(lhs, rhs, code8, code);
        } else if lhs.cat == CategoryX64::mem && rhs.cat == CategoryX64::reg {
            self.place_binary_reg_mem_and_reg(lhs, rhs, code8rev, coderev);
        } else {
            // Avoid CODEGEN_ASSERT! due to assert_call_handler signature mismatch in this crate.
            luaur_common::LUAU_DEBUGBREAK!();
        }
    }
}
