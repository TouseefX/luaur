use crate::functions::kill_ir_utils::kill_ir_function_ir_inst;
use crate::records::ir_inst::IrInst;
use crate::records::remove_dead_store_state::RemoveDeadStoreState;
use crate::records::store_reg_info::StoreRegInfo;

impl RemoveDeadStoreState {
    pub fn kill_t_value_store(&mut self, reg_info: &mut StoreRegInfo) {
        // TValue can only be killed if it is not overlayed by a partial tag/value write
        if reg_info.tvalue_inst_idx != !0u32
            && reg_info.tag_inst_idx == !0u32
            && reg_info.value_inst_idx == !0u32
        {
            if luaur_common::FFlag::LuauCodegenDseRestoreHints.get() {
                self.record_hint_before_kill(reg_info.tvalue_inst_idx);
            }

            let function = unsafe { &mut *self.function };
            let inst_ptr: *mut IrInst =
                &mut function.instructions[reg_info.tvalue_inst_idx as usize];
            kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });

            reg_info.tvalue_inst_idx = !0u32;
            reg_info.maybe_gco = false;
        }
    }
}
