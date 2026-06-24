use crate::functions::get_op_ir_data::get_op_mut;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;

#[inline]
pub fn get_op(inst: &mut IrInst, idx: u32) -> &mut IrOp {
    get_op_mut(inst, idx)
}
