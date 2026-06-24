use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn ror(&mut self, lhs: OperandX64, rhs: OperandX64) {
        self.place_shift(c"ror".as_ptr(), lhs, rhs, 1);
    }
}
