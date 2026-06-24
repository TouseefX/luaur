use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;
use luaur_common::macros::luau_unlikely::LUAU_UNLIKELY;

#[inline]
pub fn get_op_mut(inst: &mut IrInst, idx: u32) -> &mut IrOp {
    if LUAU_UNLIKELY!(idx >= inst.ops.size()) {
        inst.ops.resize(idx + 1);
    }
    &mut inst.ops[idx as usize]
}
