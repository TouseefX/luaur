use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn vmovq(&mut self, dst: OperandX64, src: OperandX64) {
        if dst.base.size() == SizeX64::xmmword {
            if !(dst.cat == CategoryX64::reg) {
                luaur_common::LUAU_DEBUGBREAK!();
            }
            if !(src.base.size() == SizeX64::qword) {
                luaur_common::LUAU_DEBUGBREAK!();
            }
            self.place_avx_c_char_operand_x_64_operand_x_64_u8_bool_u8_u8(
                c"vmovq".as_ptr(),
                dst,
                src,
                0x6e,
                true,
                0b0001,
                0b01,
            );
        } else if dst.base.size() == SizeX64::qword {
            if !(src.cat == CategoryX64::reg) {
                luaur_common::LUAU_DEBUGBREAK!();
            }
            if !(src.base.size() == SizeX64::xmmword) {
                luaur_common::LUAU_DEBUGBREAK!();
            }
            self.place_avx_c_char_operand_x_64_operand_x_64_u8_bool_u8_u8(
                c"vmovq".as_ptr(),
                src,
                dst,
                0x7e,
                true,
                0b0001,
                0b01,
            );
        } else {
            luaur_common::LUAU_DEBUGBREAK!();
        }
    }
}
