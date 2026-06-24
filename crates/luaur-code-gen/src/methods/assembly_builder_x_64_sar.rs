use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn sar(&mut self, lhs: OperandX64, rhs: OperandX64) {
        self.place_shift(b"sar".as_ptr().cast(), lhs, rhs, 7);
    }
}
