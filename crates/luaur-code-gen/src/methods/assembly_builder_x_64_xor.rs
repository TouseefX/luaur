use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn xor_(&mut self, lhs: OperandX64, rhs: OperandX64) {
        self.place_binary(
            c"xor".as_ptr(),
            lhs,
            rhs,
            0x80,
            0x81,
            0x83,
            0x30,
            0x31,
            0x32,
            0x33,
            6,
        );
    }
}
