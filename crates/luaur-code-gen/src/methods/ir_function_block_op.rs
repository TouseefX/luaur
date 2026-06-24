use crate::enums::ir_op_kind::IrOpKind;
use crate::records::ir_block::IrBlock;
use crate::records::ir_function::IrFunction;
use crate::records::ir_op::IrOp;

impl IrFunction {
    pub fn block_op(&mut self, op: IrOp) -> &mut IrBlock {
        assert!(op.kind() == IrOpKind::Block);
        &mut self.blocks[op.index() as usize]
    }
}
