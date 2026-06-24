use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn vdpps(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64, mask: u8) {
        self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
            b"vdpps\0".as_ptr() as *const core::ffi::c_char,
            dst,
            src1,
            src2,
            // C++: placeAvx("vdpps", dst, src1, src2, mask, 0x40, false,
            //               AVX_0F3A, AVX_66);
            // mode = AVX_0F3A (0x3A -> 0b00011), prefix = AVX_66 (0x66 -> 0b01).
            mask,
            0x40,
            false,
            0x3A, // AVX_0F3A
            0x66, // AVX_66
        );
    }
}
