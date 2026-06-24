use crate::functions::vm_reg_op::vm_reg_op;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_op::IrOp;

impl ConstPropState {
    pub fn invalidate_tag(&mut self, reg_op: IrOp) {
        let reg = vm_reg_op(reg_op);
        if reg > self.max_reg {
            self.max_reg = reg;
        }

        let reg_idx = reg as usize;
        let reg_ptr: *mut crate::records::register_info::RegisterInfo = &mut self.regs[reg_idx];
        unsafe {
            self.invalidate_register_info_bool_bool(&mut *reg_ptr, true, false);
        }
    }
}
