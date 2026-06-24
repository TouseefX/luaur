use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn place_binary_reg_and_reg_mem(
        &mut self,
        lhs: OperandX64,
        rhs: OperandX64,
        code8: u8,
        code: u8,
    ) {
        if !(lhs.cat == CategoryX64::reg
            && (rhs.cat == CategoryX64::reg || rhs.cat == CategoryX64::mem))
        {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        let size = if rhs.cat == CategoryX64::reg {
            rhs.base.size()
        } else {
            rhs.memSize
        };
        if !(lhs.base.size() == size) {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        if !(size == SizeX64::byte
            || size == SizeX64::word
            || size == SizeX64::dword
            || size == SizeX64::qword)
        {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        if size == SizeX64::word {
            self.place(0x66);
        }

        self.place_rex_register_x_64_operand_x_64(lhs.base, rhs);
        self.place(if size == SizeX64::byte { code8 } else { code });
        self.place_reg_and_mod_reg_mem(lhs, rhs, 0);

        self.commit();
    }
}
