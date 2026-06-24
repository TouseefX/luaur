use crate::enums::ir_condition::IrCondition;
use crate::enums::ir_op_kind::IrOpKind;
use crate::records::ir_op::IrOp;

pub fn condition_op(op: IrOp) -> IrCondition {
    debug_assert!(op.kind() == IrOpKind::Condition);
    let index = op.index();
    if index < IrCondition::Count as u32 {
        unsafe { core::mem::transmute(index as u8) }
    } else {
        IrCondition::Count
    }
}
