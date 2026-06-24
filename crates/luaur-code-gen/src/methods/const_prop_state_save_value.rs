use crate::enums::ir_op_kind::IrOpKind;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_op::IrOp;
use crate::records::register_info::RegisterInfo;

impl ConstPropState {
    pub fn save_value(&mut self, op: IrOp, value: IrOp) {
        CODEGEN_ASSERT!(value.kind() == IrOpKind::Constant);

        if let Some(info) = self.try_get_register_info(op) {
            unsafe {
                if (*info).value != value {
                    (*info).value = value;

                    if !luaur_common::FFlag::LuauCodegenExtraTableOpts.get() {
                        (*info).known_not_readonly_deprecated = false;
                        (*info).known_no_metatable_deprecated = false;
                        (*info).known_table_array_size_deprecated = -1;
                    }

                    (*info).version += 1;
                }
            }
        }
    }
}
