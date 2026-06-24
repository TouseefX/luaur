use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn add(&mut self, lhs: OperandX64, rhs: OperandX64) {
        self.place_binary(
            b"add\0".as_ptr() as *const core::ffi::c_char,
            lhs,
            rhs,
            0x80,
            0x81,
            0x83,
            0x00,
            0x01,
            0x02,
            0x03,
            0,
        );
    }
}
