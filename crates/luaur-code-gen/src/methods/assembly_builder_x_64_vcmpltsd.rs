use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn vcmpltsd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
        // C++: placeAvx("vcmpltsd", dst, src1, src2, 0x01, 0xc2, false, AVX_0F, AVX_F2);
        // imm8-carrying overload: imm8=0x01, code(opcode)=0xc2.
        self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
            c"vcmpltsd".as_ptr(),
            dst,
            src1,
            src2,
            0x01,
            0xc2,
            false,
            0x0F, // AVX_0F
            0xF2, // AVX_F2
        );
    }
}
