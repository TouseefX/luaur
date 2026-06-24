impl crate::records::const_prop_state::ConstPropState {
    pub fn invalidate_table_array_size(&mut self) {
        if luaur_common::FFlag::LuauCodegenExtraTableOpts.get() {
            self.inst_array_size.clear();
        } else {
            let max_reg = self.max_reg;
            for i in 0..=max_reg {
                let idx = i as usize;
                let reg_ptr: *mut crate::records::register_info::RegisterInfo = &mut self.regs[idx];
                unsafe {
                    self.invalidate_table_array_size_register_info(&mut *reg_ptr);
                }
            }
        }

        self.invalidate_heap_table_data();
    }
}
