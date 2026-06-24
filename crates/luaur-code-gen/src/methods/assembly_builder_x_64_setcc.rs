use crate::enums::category_x_64::CategoryX64;
use crate::enums::condition_x_64::ConditionX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn setcc(&mut self, cond: ConditionX64, op: OperandX64) {
        let size = if op.cat == CategoryX64::reg {
            op.base.size()
        } else {
            op.memSize
        };

        if !(size == SizeX64::byte) {
            luaur_common::LUAU_DEBUGBREAK!();
        }

        if self.log_text {
            static SETCC_TEXT: [&[u8]; 26] = [
                b"seto\0",
                b"setno\0",
                b"setc\0",
                b"setnc\0",
                b"setb\0",
                b"setbe\0",
                b"seta\0",
                b"setae\0",
                b"sete\0",
                b"setl\0",
                b"setle\0",
                b"setg\0",
                b"setge\0",
                b"setnb\0",
                b"setnbe\0",
                b"setna\0",
                b"setnae\0",
                b"setne\0",
                b"setnl\0",
                b"setnle\0",
                b"setng\0",
                b"setnge\0",
                b"setz\0",
                b"setnz\0",
                b"setp\0",
                b"setnp\0",
            ];

            let cond_idx = cond as usize;
            self.log_c_char_operand_x_64(
                SETCC_TEXT[cond_idx].as_ptr() as *const core::ffi::c_char,
                op,
            );
        }

        self.place_rex_operand_x_64(op);
        self.place(0x0f);

        static CODE_FOR_CONDITION: [u8; 26] = [
            0x00, 0x01, 0x02, 0x03, 0x02, 0x06, 0x07, 0x03, 0x04, 0x0c, 0x0e, 0x0f, 0x0d, 0x03,
            0x07, 0x06, 0x02, 0x05, 0x0d, 0x0f, 0x0e, 0x0c, 0x04, 0x05, 0x0a, 0x0b,
        ];

        self.place(0x90 | CODE_FOR_CONDITION[cond as usize]);
        self.place_mod_reg_mem(op, 0, 0);
        self.commit();
    }
}
