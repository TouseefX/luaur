use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl AssemblyBuilderX64 {
    pub fn vblendvps(
        &mut self,
        dst: RegisterX64,
        src1: RegisterX64,
        src2: OperandX64,
        mask: RegisterX64,
    ) {
        // bits [7:4] of imm8 are used to select register for operand 4
        self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
            b"vblendvps\0".as_ptr() as *const core::ffi::c_char,
            OperandX64::reg(RegisterX64::noreg),
            src1.into(),
            src2,
            mask.index() << 4,
            0x4a,
            false,
            0x4a,
            0x66,
        );
    }
}
