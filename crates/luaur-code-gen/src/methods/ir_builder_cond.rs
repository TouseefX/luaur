use crate::enums::ir_condition::IrCondition;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;

impl IrBuilder {
    pub fn cond(&mut self, cond: IrCondition) -> IrOp {
        IrOp::ir_op_ir_op_kind_u32(
            crate::enums::ir_op_kind::IrOpKind::Condition,
            cond as u8 as u32,
        )
    }
}
