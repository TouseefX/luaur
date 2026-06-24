use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn place_binary_reg_mem_and_imm(
        &mut self,
        lhs: OperandX64,
        rhs: OperandX64,
        code8: u8,
        code: u8,
        code_imm8: u8,
        opreg: u8,
    ) {
        if !(lhs.cat == CategoryX64::reg || lhs.cat == CategoryX64::mem) {
            luaur_common::LUAU_DEBUGBREAK!();
        }
        if !(rhs.cat == CategoryX64::imm) {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        let size = if lhs.cat == CategoryX64::reg {
            lhs.base.size()
        } else {
            lhs.memSize
        };

        if !(size == SizeX64::byte || size == SizeX64::dword || size == SizeX64::qword) {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        self.place_rex_operand_x_64(lhs);

        if size == SizeX64::byte {
            self.place(code8);
            self.place_mod_reg_mem(lhs, opreg, 1);
            self.place_imm_8(rhs.imm);
        } else {
            if !(size == SizeX64::dword || size == SizeX64::qword) {
                luaur_common::LUAU_DEBUGBREAK!();
            }

            if (rhs.imm as i8) as i32 == rhs.imm && code != code_imm8 {
                self.place(code_imm8);
                self.place_mod_reg_mem(lhs, opreg, 1);
                self.place_imm_8(rhs.imm);
            } else {
                self.place(code);
                self.place_mod_reg_mem(lhs, opreg, 4);
                self.place_imm_32(rhs.imm);
            }
        }

        self.commit();
    }
}
