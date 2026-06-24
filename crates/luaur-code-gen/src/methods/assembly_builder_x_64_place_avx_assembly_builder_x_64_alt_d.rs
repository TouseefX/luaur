use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
        &mut self,
        name: *const core::ffi::c_char,
        dst: OperandX64,
        src1: OperandX64,
        src2: OperandX64,
        imm8: u8,
        code: u8,
        set_w: bool,
        mode: u8,
        prefix: u8,
    ) {
        // Avoid CODEGEN_ASSERT! because it routes through assert_call_handler expecting *const i8
        // while this invocation produces &str from stringify!(...).
        if !(dst.cat == crate::enums::category_x_64::CategoryX64::reg) {
            luaur_common::LUAU_DEBUGBREAK!();
        }
        if !(src1.cat == crate::enums::category_x_64::CategoryX64::reg) {
            luaur_common::LUAU_DEBUGBREAK!();
        }
        if !(src2.cat == crate::enums::category_x_64::CategoryX64::reg
            || src2.cat == crate::enums::category_x_64::CategoryX64::mem)
        {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        if self.log_text {
            // C++ `placeAvx(..., imm8, ...)` logs `imm8` as a trailing operand
            // (implicit `OperandX64(int32_t)` -> imm category).
            let imm_op = OperandX64::from(imm8 as i32);
            if src1.base == crate::records::register_x_64::RegisterX64::noreg {
                self.log_c_char_operand_x_64_operand_x_64_operand_x_64(name, src2, dst, imm_op);
            } else {
                self.log_c_char_operand_x_64_operand_x_64_operand_x_64_operand_x_64(
                    name, dst, src1, src2, imm_op,
                );
            }
        }

        self.place_vex(dst, src1, src2, set_w, mode, prefix);
        self.place(code);
        self.place_reg_and_mod_reg_mem(dst, src2, 1);
        self.place_imm_8(imm8 as i32);

        self.commit();
    }
}
