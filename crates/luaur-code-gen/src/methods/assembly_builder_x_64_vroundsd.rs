use crate::enums::rounding_mode_x_64::RoundingModeX64;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn vroundsd(
        &mut self,
        dst: OperandX64,
        src1: OperandX64,
        src2: OperandX64,
        rounding_mode: RoundingModeX64,
    ) {
        // C++: placeAvx("vroundsd", dst, src1, src2,
        //               uint8_t(roundingMode) | kRoundingPrecisionInexact,
        //               0x0b, false, AVX_0F3A, AVX_66);
        // kRoundingPrecisionInexact is 0b1000 (0x08), and the opcode map is
        // AVX_0F3A (normalized from 0x3A), NOT 0.
        const K_ROUNDING_PRECISION_INEXACT: u8 = 0x08;
        const CODE: u8 = 0x0b;
        const AVX_0F3A: u8 = 0x3A;
        const AVX_66: u8 = 0x66;

        self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
            b"vroundsd\0".as_ptr() as *const core::ffi::c_char,
            dst,
            src1,
            src2,
            (rounding_mode as u8) | K_ROUNDING_PRECISION_INEXACT,
            CODE,
            false,
            AVX_0F3A,
            AVX_66,
        );
    }
}
