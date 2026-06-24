use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl AssemblyBuilderX64 {
    pub fn vpshufps(&mut self, dst: RegisterX64, src1: RegisterX64, src2: OperandX64, shuffle: u8) {
        self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
            c"vpshufps".as_ptr(),
            OperandX64::reg(dst),
            OperandX64::reg(src1),
            src2,
            shuffle,
            0xc6,
            false,
            0x0F,
            0x00,
        );
    }
}
