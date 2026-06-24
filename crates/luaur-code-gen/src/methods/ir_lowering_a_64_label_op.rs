use crate::records::ir_lowering_a_64::IrLoweringA64;
use crate::records::ir_op::IrOp;
use crate::records::label::Label;

impl IrLoweringA64 {
    pub fn ir_lowering_a_64_label_op(&mut self, op: IrOp) -> &mut Label {
        &mut self.ir_lowering_a_64_block_op(op).label
    }
}
