use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl AssemblyBuilderX64 {
    pub fn place_shift(
        &mut self,
        name: *const core::ffi::c_char,
        lhs: OperandX64,
        rhs: OperandX64,
        opreg: u8,
    ) {
        if self.log_text {
            self.log_c_char_operand_x_64_operand_x_64(name, lhs, rhs);
        }

        let cl = RegisterX64 {
            bits: (1u8 << RegisterX64::INDEX_SHIFT) | SizeX64::byte as u8,
        };

        if !(lhs.cat == CategoryX64::reg || lhs.cat == CategoryX64::mem) {
            luaur_common::LUAU_DEBUGBREAK!();
        }
        if !(rhs.cat == CategoryX64::imm || (rhs.cat == CategoryX64::reg && rhs.base == cl)) {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        let size = lhs.base.size();

        self.place_rex_register_x_64(lhs.base);

        if rhs.cat == CategoryX64::imm && rhs.imm == 1 {
            self.place(if size == SizeX64::byte { 0xd0 } else { 0xd1 });
            self.place_mod_reg_mem(lhs, opreg, 0);
        } else if rhs.cat == CategoryX64::imm {
            if !((rhs.imm as i8) as i32 == rhs.imm) {
                luaur_common::LUAU_DEBUGBREAK!();
            }

            self.place(if size == SizeX64::byte { 0xc0 } else { 0xc1 });
            self.place_mod_reg_mem(lhs, opreg, 1);
            self.place_imm_8(rhs.imm);
        } else {
            self.place(if size == SizeX64::byte { 0xd2 } else { 0xd3 });
            self.place_mod_reg_mem(lhs, opreg, 0);
        }

        self.commit();
    }
}
