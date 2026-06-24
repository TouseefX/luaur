use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::ir_op::IrOp;
use crate::records::operand_x_64::OperandX64;

impl IrLoweringX64 {
    pub fn store_double_as_float(&mut self, dst: OperandX64, src: IrOp) {
        panic!("ir_lowering_x_64_store_double_as_float not yet translated");
    }
}
