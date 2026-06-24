use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::ir_op::IrOp;
use crate::records::label::Label;

impl IrLoweringX64 {
    pub fn label_op(&self, op: IrOp) -> &mut Label {
        &mut self.block_op(op).label
    }
}
