use crate::enums::ir_block_kind::IrBlockKind;
use crate::records::ir_block::IrBlock;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;

impl IrBuilder {
    pub fn is_internal_block(&self, block: IrOp) -> bool {
        let target: &IrBlock = &self.function.blocks[block.index() as usize];
        target.kind == IrBlockKind::Internal
    }
}
