use crate::functions::get_op_ir_data::get_op_mut;

/// C++ `OP_A(inst)` — the instruction's first operand.
#[inline]
pub fn op_a(inst: &mut crate::records::ir_inst::IrInst) -> crate::records::ir_op::IrOp {
    *get_op_mut(inst, 0)
}
