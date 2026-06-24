use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_op::IrOp;

use crate::functions::vm_reg_op::vm_reg_op;

impl ConstPropState {
    pub fn invalidate_ir_op(&mut self, _reg_op: IrOp) {
        // TODO: use maxstacksize from Proto
        let reg_op = _reg_op;

        let vm_reg = vm_reg_op(reg_op);
        let max_reg = if vm_reg > self.max_reg {
            vm_reg
        } else {
            self.max_reg
        };
        self.max_reg = max_reg;

        // Avoid borrowing `self` (and `self.regs[idx]`) mutably more than once:
        // - take a raw pointer to the register slot
        // - perform the invalidation call using the raw pointer as a mutable ref
        let reg_idx = vm_reg as usize;
        let reg_ptr: *mut crate::records::register_info::RegisterInfo = &mut self.regs[reg_idx];
        unsafe {
            self.invalidate_register_info_bool_bool(&mut *reg_ptr, true, true);
        }
    }
}
