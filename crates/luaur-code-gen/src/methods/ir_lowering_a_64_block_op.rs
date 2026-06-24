use crate::records::ir_block::IrBlock;
use crate::records::ir_lowering_a_64::IrLoweringA64;
use crate::records::ir_op::IrOp;

impl IrLoweringA64 {
    pub fn ir_lowering_a_64_block_op(&self, op: IrOp) -> &mut IrBlock {
        unsafe { (*self.function).block_op(op) }
    }
}
