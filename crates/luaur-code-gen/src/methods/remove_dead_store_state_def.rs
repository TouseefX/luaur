use crate::functions::vm_reg_op::vm_reg_op;
use crate::records::ir_op::IrOp;
use crate::records::remove_dead_store_state::RemoveDeadStoreState;

impl RemoveDeadStoreState {
    pub fn def(&mut self, op: IrOp, offset: i32) {
        let reg = vm_reg_op(op) + offset;
        self.def_reg(reg as u8);
    }
}
