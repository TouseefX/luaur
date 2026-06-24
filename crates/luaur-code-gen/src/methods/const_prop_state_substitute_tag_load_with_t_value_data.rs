use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::substitute::substitute;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::op_a::op_a;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_inst::IrInst;

impl ConstPropState {
    pub fn substitute_tag_load_with_t_value_data(
        &mut self,
        build: &mut IrBuilder,
        load_inst: &mut IrInst,
    ) -> bool {
        CODEGEN_ASSERT!(op_a(load_inst).kind() == IrOpKind::VmReg);

        if let Some(prev_idx) =
            self.get_previous_versioned_load_index(IrCmd::LOAD_TVALUE, op_a(load_inst))
        {
            if let Some(tag) = self.inst_tag.find(&unsafe { *prev_idx }) {
                if *tag != 0xff {
                    let replacement = build.const_tag(*tag);
                    unsafe { substitute(&mut *self.function, load_inst, replacement) };
                    return true;
                }
            }
        }

        false
    }
}
