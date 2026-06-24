use crate::functions::get_op_ir_data::get_op_mut;

#[inline]
pub fn op_b(mut inst: crate::records::ir_inst::IrInst) -> crate::records::ir_op::IrOp {
    *get_op_mut(&mut inst, 1)
}
