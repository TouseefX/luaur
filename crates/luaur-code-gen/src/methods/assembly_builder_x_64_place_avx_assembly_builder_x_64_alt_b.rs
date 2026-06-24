use crate::enums::category_x_64::CategoryX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn place_avx_c_char_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
        &mut self,
        name: *const core::ffi::c_char,
        dst: OperandX64,
        src: OperandX64,
        code: u8,
        coderev: u8,
        set_w: bool,
        mode: u8,
        prefix: u8,
    ) {
        // Avoid CODEGEN_ASSERT! due to assert_call_handler signature mismatch in this crate.
        if !((dst.cat == CategoryX64::mem && src.cat == CategoryX64::reg)
            || (dst.cat == CategoryX64::reg && src.cat == CategoryX64::mem)
            || (dst.cat == CategoryX64::reg && src.cat == CategoryX64::reg))
        {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        if self.log_text {
            self.log_c_char_operand_x_64_operand_x_64(name, dst, src);
        }

        if dst.cat == CategoryX64::mem {
            self.place_vex(
                src,
                OperandX64::reg(crate::records::register_x_64::RegisterX64::noreg),
                dst,
                set_w,
                mode,
                prefix,
            );
            self.place(coderev);
            self.place_reg_and_mod_reg_mem(src, dst, 0);
        } else {
            self.place_vex(
                dst,
                OperandX64::reg(crate::records::register_x_64::RegisterX64::noreg),
                src,
                set_w,
                mode,
                prefix,
            );
            self.place(code);
            self.place_reg_and_mod_reg_mem(dst, src, 0);
        }

        self.commit();
    }
}
