use crate::enums::condition_x_64::ConditionX64;
use crate::records::ir_block::IrBlock;
use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::ir_op::IrOp;

impl IrLoweringX64 {
    pub fn jump_or_abort_on_undef_ir_op_u32_ir_block(
        &mut self,
        target: IrOp,
        index: u32,
        next: &IrBlock,
    ) {
        self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
            ConditionX64::Count,
            target,
            index,
            next,
        );
    }
}
