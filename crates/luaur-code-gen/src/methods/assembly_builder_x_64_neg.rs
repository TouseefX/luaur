use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn neg(&mut self, op: OperandX64) {
        self.place_unary_mod_reg_mem(c"neg".as_ptr(), op, 0xf6, 0xf7, 3);
    }
}
