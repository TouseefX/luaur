use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn test(&mut self, lhs: OperandX64, rhs: OperandX64) {
        self.place_binary(
            c"test".as_ptr(),
            lhs,
            rhs,
            0xf6,
            0xf7,
            0xf7,
            0x84,
            0x85,
            0x84,
            0x85,
            0,
        );
    }
}
