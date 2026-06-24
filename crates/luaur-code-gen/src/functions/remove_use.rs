use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::remove_block_use::remove_block_use;
use crate::functions::remove_inst_use::remove_inst_use;
use crate::records::ir_function::IrFunction;
use crate::records::ir_op::IrOp;

pub fn remove_use(function: &mut IrFunction, op: IrOp) {
    if op.kind() == IrOpKind::Inst {
        remove_inst_use(function, op.index());
    } else if op.kind() == IrOpKind::Block {
        remove_block_use(function, op.index());
    }
}
