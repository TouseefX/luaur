use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl AssemblyBuilderX64 {
    pub fn vblendvpd(
        &mut self,
        dst: RegisterX64,
        src1: RegisterX64,
        src2: OperandX64,
        mask: RegisterX64,
    ) {
        // bits [7:4] of imm8 are used to select register for operand 4
        // C++: placeAvx("vblendvpd", dst, src1, src2, mask.index << 4, 0x4b,
        //               false, AVX_0F3A, AVX_66);
        // This needs the imm8-carrying overload: imm8 = mask.index << 4,
        // code(opcode) = 0x4b, mode = AVX_0F3A (0x3A -> 0b00011).
        self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
            b"vblendvpd\0".as_ptr() as *const core::ffi::c_char,
            OperandX64::reg(dst),
            OperandX64::reg(src1),
            src2,
            mask.index() << 4,
            0x4b,
            false,
            0x3A, // AVX_0F3A
            0x66, // AVX_66
        );
    }
}
