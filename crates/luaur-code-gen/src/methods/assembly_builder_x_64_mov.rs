use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::op_plus_reg::OP_PLUS_REG;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl AssemblyBuilderX64 {
    pub fn mov(&mut self, lhs: OperandX64, rhs: OperandX64) {
        if self.log_text {
            self.log_c_char_operand_x_64_operand_x_64(c"mov".as_ptr(), lhs, rhs);
        }

        if lhs.cat == CategoryX64::reg && rhs.cat == CategoryX64::imm {
            let size = lhs.base.size();

            self.place_rex_register_x_64(lhs.base);

            if size == SizeX64::byte {
                self.place(OP_PLUS_REG(0xb0, lhs.base.index()));
                self.place_imm_8(rhs.imm);
            } else if size == SizeX64::word {
                self.place(0x66);
                self.place(OP_PLUS_REG(0xb8, lhs.base.index()));
                self.place_imm_16(rhs.imm as i16);
            } else if size == SizeX64::dword {
                self.place(OP_PLUS_REG(0xb8, lhs.base.index()));
                self.place_imm_32(rhs.imm);
            } else {
                // qword
                self.place(OP_PLUS_REG(0xb8, lhs.base.index()));
                self.place_imm_64(rhs.imm as i64);
            }
        } else if lhs.cat == CategoryX64::mem && rhs.cat == CategoryX64::imm {
            let size = lhs.memSize;

            self.place_rex_operand_x_64(lhs);

            if size == SizeX64::byte {
                self.place(0xc6);
                self.place_mod_reg_mem(lhs, 0, 1);
                self.place_imm_8(rhs.imm);
            } else if size == SizeX64::word {
                self.place(0x66);
                self.place(0xc7);
                self.place_mod_reg_mem(lhs, 0, 2);
                self.place_imm_16(rhs.imm as i16);
            } else {
                // dword or qword: both encoded with imm32 in this routine
                self.place(0xc7);
                self.place_mod_reg_mem(lhs, 0, 4);
                self.place_imm_32(rhs.imm);
            }
        } else if lhs.cat == CategoryX64::reg
            && (rhs.cat == CategoryX64::reg || rhs.cat == CategoryX64::mem)
        {
            self.place_binary_reg_and_reg_mem(lhs, rhs, 0x8a, 0x8b);
        } else if lhs.cat == CategoryX64::mem && rhs.cat == CategoryX64::reg {
            self.place_binary_reg_mem_and_reg(lhs, rhs, 0x88, 0x89);
        } else {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        self.commit();
    }
}
