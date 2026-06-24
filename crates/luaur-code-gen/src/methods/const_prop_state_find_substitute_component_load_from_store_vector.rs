use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::op_b::op_b;
use crate::macros::op_c::op_c;
use crate::macros::op_d::op_d;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;

impl ConstPropState {
    pub fn find_substitute_component_load_from_store_vector(
        &mut self,
        _build: &mut IrBuilder,
        vm_reg: IrOp,
        offset: i32,
    ) -> Option<IrOp> {
        let versioned_load = self.versioned_vm_reg_load_ir_cmd_ir_op(IrCmd::LOAD_FLOAT, vm_reg);

        if let Some(prev_idx) = self.get_previous_inst_index(&versioned_load) {
            let function = unsafe { &mut *self.function };
            let store = &function.instructions[unsafe { *prev_idx } as usize];

            CODEGEN_ASSERT!(store.cmd == IrCmd::STORE_VECTOR);

            let arg_op = if offset == 0 {
                op_b(store.clone())
            } else if offset == 4 {
                op_c(store.clone())
            } else if offset == 8 {
                op_d(store.clone())
            } else {
                return None;
            };

            let function = unsafe { &mut *self.function };
            let arg = function.as_inst_op(arg_op);

            if !arg.is_null() {
                let arg_cmd = unsafe { &*arg }.cmd;
                if arg_cmd == IrCmd::LOAD_FLOAT
                    || arg_cmd == IrCmd::BUFFER_READF32
                    || arg_cmd == IrCmd::NUM_TO_FLOAT
                    || arg_cmd == IrCmd::UINT_TO_FLOAT
                {
                    return Some(arg_op);
                }
            } else if arg_op.kind() == IrOpKind::Constant {
                let double_val = function.double_op(arg_op);
                return Some(_build.const_double(double_val as f64));
            }
        }

        None
    }
}
