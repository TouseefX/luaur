use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;

impl IrBuilder {
    pub fn begin_block(&mut self, block: IrOp) {
        let target = &mut self.function.blocks[block.index() as usize];
        self.active_block_idx = block.index();

        CODEGEN_ASSERT!(
            target.start == !0u32 || target.start == self.function.instructions.len() as u32
        );

        target.start = self.function.instructions.len() as u32;
        target.sortkey = target.start;

        self.in_terminated_block = false;
    }
}
