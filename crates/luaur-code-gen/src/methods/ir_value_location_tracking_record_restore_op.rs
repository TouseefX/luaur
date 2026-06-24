use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::get_cmd_value_kind::get_cmd_value_kind;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::op_a::op_a;
use crate::records::ir_data::k_invalid_inst_idx;
use crate::records::ir_op::IrOp;
use crate::records::ir_value_location_tracking::IrValueLocationTracking;
use crate::records::value_restore_location::ValueRestoreLocation;
use luaur_common::FFlag;

impl IrValueLocationTracking {
    pub fn record_restore_op(&mut self, inst_idx: u32, location: IrOp) {
        let function = unsafe { &mut *self.function };
        let inst = function.instructions[inst_idx as usize].clone();

        if location.kind() == IrOpKind::VmReg {
            let reg = vm_reg_op(location);

            if reg > self.max_reg {
                self.max_reg = reg;
            }

            let captured = (function.cfg.captured.regs[reg as usize / 64]
                & (1u64 << (reg as usize % 64)))
                != 0;

            if !captured {
                function.record_restore_location(
                    inst_idx,
                    ValueRestoreLocation {
                        op: location,
                        kind: get_cmd_value_kind(inst.cmd),
                        conversion_cmd: IrCmd::NOP,
                        lazy: false,
                    },
                );
            }

            self.vm_reg_value[reg as usize] = inst_idx;

            if FFlag::LuauCodegenForwardRematerialize.get() {
                CODEGEN_ASSERT!(self.vm_reg_dependent[reg as usize] == k_invalid_inst_idx);
            }

            let mut inst_for_op = inst.clone();
            if self.can_be_rematerialized(inst.cmd)
                && op_a(&mut inst_for_op).kind() == IrOpKind::Inst
            {
                let dep_inst_idx = op_a(&mut inst_for_op).index();

                if !captured {
                    function.record_restore_location(
                        dep_inst_idx,
                        ValueRestoreLocation {
                            op: location,
                            kind: get_cmd_value_kind(inst.cmd),
                            conversion_cmd: inst.cmd,
                            lazy: false,
                        },
                    );
                }

                if FFlag::LuauCodegenForwardRematerialize.get() {
                    self.vm_reg_dependent[reg as usize] = dep_inst_idx;
                }
            }
        } else if location.kind() == IrOpKind::VmConst {
            function.record_restore_location(
                inst_idx,
                ValueRestoreLocation {
                    op: location,
                    kind: get_cmd_value_kind(inst.cmd),
                    conversion_cmd: IrCmd::NOP,
                    lazy: false,
                },
            );
        }
    }
}
