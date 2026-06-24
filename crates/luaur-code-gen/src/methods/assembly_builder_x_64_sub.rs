use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn sub(&mut self, lhs: OperandX64, rhs: OperandX64) {
        self.place_binary(
            b"sub\0" as *const _ as *const core::ffi::c_char,
            lhs,
            rhs,
            0x80,
            0x81,
            0x83,
            0x28,
            0x29,
            0x2a,
            0x2b,
            5,
        );
    }
}
