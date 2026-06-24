use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn or_(&mut self, lhs: OperandX64, rhs: OperandX64) {
        self.place_binary(
            b"or\0" as *const _ as *const core::ffi::c_char,
            lhs,
            rhs,
            0x80,
            0x81,
            0x83,
            0x08,
            0x09,
            0x0a,
            0x0b,
            1,
        );
    }
}
