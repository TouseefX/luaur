use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn shr(&mut self, lhs: OperandX64, rhs: OperandX64) {
        self.place_shift(b"shr\0".as_ptr() as *const core::ffi::c_char, lhs, rhs, 5);
    }
}
