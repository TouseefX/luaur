use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::enums::ir_value_kind::IrValueKind;
use crate::functions::get_cmd_value_kind::get_cmd_value_kind;
use crate::records::ir_op::IrOp;
use crate::records::remove_dead_store_state::RemoveDeadStoreState;
use crate::records::store_location_hint::StoreLocationHint;

impl RemoveDeadStoreState {
    pub fn record_hint_before_kill(&mut self, store_inst_idx: u32) {
        let function = unsafe { &mut *self.function };

        let store_cmd = function.instructions[store_inst_idx as usize].cmd;

        let dest: IrOp = function.instructions[store_inst_idx as usize].ops[0];

        if dest.kind() != IrOpKind::VmReg {
            return;
        }

        let mut value = IrOp::default();
        let mut kind = IrValueKind::Unknown;

        match store_cmd {
            IrCmd::STORE_DOUBLE => {
                value = function.instructions[store_inst_idx as usize].ops[1];
                kind = IrValueKind::Double;
            }
            IrCmd::STORE_INT => {
                value = function.instructions[store_inst_idx as usize].ops[1];
                kind = IrValueKind::Int;
            }
            IrCmd::STORE_INT64 => {
                value = function.instructions[store_inst_idx as usize].ops[1];
                kind = IrValueKind::Int64;
            }
            IrCmd::STORE_POINTER => {
                value = function.instructions[store_inst_idx as usize].ops[1];
                kind = IrValueKind::Pointer;
            }
            IrCmd::STORE_TVALUE => {
                value = function.instructions[store_inst_idx as usize].ops[1];
                kind = IrValueKind::Tvalue;
            }
            IrCmd::STORE_SPLIT_TVALUE => {
                value = function.instructions[store_inst_idx as usize].ops[2];

                if value.kind() == IrOpKind::Inst {
                    let inst_cmd = function.inst_op(value).cmd;
                    kind = get_cmd_value_kind(inst_cmd);
                }

                if kind == IrValueKind::Unknown {
                    return;
                }
            }
            IrCmd::STORE_VECTOR => {
                // multi-component, not useful as a single-value restore hint
                return;
            }
            _ => return,
        }

        if value.kind() != IrOpKind::Inst {
            return;
        }

        function.record_store_location_hint(
            store_inst_idx,
            StoreLocationHint {
                op: dest,
                inst_idx: value.index(),
                kind,
            },
        );
    }
}
