use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl AssemblyBuilderX64 {
    pub fn vpinsrd(&mut self, dst: RegisterX64, src1: RegisterX64, src2: OperandX64, offset: u8) {
        // C++: placeAvx("vpinsrd", dst, src1, src2, offset, 0x22, false,
        //               AVX_0F3A, AVX_66);
        // dst is the real destination register (not noreg), and the opcode map
        // is AVX_0F3A (0x3A -> 0b00011), not 0xF3.
        self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
            b"vpinsrd\0".as_ptr() as *const core::ffi::c_char,
            OperandX64::reg(dst),
            OperandX64::reg(src1),
            src2,
            offset,
            0x22,
            false,
            0x3A, // AVX_0F3A
            0x66, // AVX_66
        );
    }
}
