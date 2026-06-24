use crate::enums::ir_op_kind::IrOpKind;
use crate::records::ir_function::IrFunction;
use crate::records::ir_op::IrOp;

pub fn add_use(function: &mut IrFunction, op: IrOp) {
    if op.kind() == IrOpKind::Inst {
        function.instructions[op.index() as usize].use_count += 1;
    } else if op.kind() == IrOpKind::Block {
        function.blocks[op.index() as usize].use_count += 1;
    }
}
