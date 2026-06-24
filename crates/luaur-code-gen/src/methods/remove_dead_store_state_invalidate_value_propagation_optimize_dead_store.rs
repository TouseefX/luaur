use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::any_argument_match::any_argument_match;
use crate::records::ir_function::IrFunction;
use crate::records::remove_dead_store_state::RemoveDeadStoreState;
use crate::records::store_reg_info::StoreRegInfo;

impl RemoveDeadStoreState {
    // Marks pending stores as non-propagating to prevent moving their uses into VM exit blocks
    pub fn invalidate_value_propagation_store_reg_info(&mut self, reg_info: &mut StoreRegInfo) {
        let function: *mut IrFunction = self.function;

        let has_inst_arg = |idx: u32| -> bool {
            any_argument_match(
                unsafe { &(&(*function).instructions)[idx as usize] },
                |op| op.kind() == IrOpKind::Inst,
            )
        };

        if reg_info.tag_inst_idx != !0u32 && has_inst_arg(reg_info.tag_inst_idx) {
            self.non_propagating_store.insert(reg_info.tag_inst_idx);
        }

        if reg_info.value_inst_idx != !0u32 && has_inst_arg(reg_info.value_inst_idx) {
            self.non_propagating_store.insert(reg_info.value_inst_idx);
        }

        if reg_info.tvalue_inst_idx != !0u32 && has_inst_arg(reg_info.tvalue_inst_idx) {
            self.non_propagating_store.insert(reg_info.tvalue_inst_idx);
        }
    }
}
