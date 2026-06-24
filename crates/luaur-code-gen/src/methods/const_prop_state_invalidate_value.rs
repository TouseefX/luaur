impl crate::records::const_prop_state::ConstPropState {
    pub fn invalidate_value(&mut self, reg_op: crate::records::ir_op::IrOp) {
        let reg_index = crate::functions::vm_reg_op::vm_reg_op(reg_op);
        if reg_index > self.max_reg {
            self.max_reg = reg_index;
        }
        let reg_ptr: *mut crate::records::register_info::RegisterInfo =
            &mut self.regs[reg_index as usize];
        unsafe {
            self.invalidate_register_info_bool_bool(&mut *reg_ptr, false, true);
        }
    }
}
