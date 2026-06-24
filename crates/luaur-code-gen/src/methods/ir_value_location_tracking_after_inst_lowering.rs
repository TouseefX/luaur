use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::enums::ir_value_kind::IrValueKind;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::macros::op_a::op_a;
use crate::macros::op_b::op_b;
use crate::macros::op_c::op_c;
use crate::records::ir_data::k_invalid_inst_idx;
use crate::records::ir_inst::IrInst;
use crate::records::ir_value_location_tracking::IrValueLocationTracking;
use crate::records::value_restore_location::ValueRestoreLocation;
use luaur_common::FFlag;

impl IrValueLocationTracking {
    pub fn after_inst_lowering(&mut self, inst: &mut IrInst, inst_idx: u32) {
        match inst.cmd {
            IrCmd::LOAD_TAG
            | IrCmd::LOAD_POINTER
            | IrCmd::LOAD_DOUBLE
            | IrCmd::LOAD_INT
            | IrCmd::LOAD_INT64
            | IrCmd::LOAD_TVALUE => {
                if op_a(inst).kind() == IrOpKind::VmReg {
                    self.invalidate_restore_op(op_a(inst), false);
                }

                self.record_restore_op(inst_idx, op_a(inst));
            }

            IrCmd::STORE_POINTER
            | IrCmd::STORE_DOUBLE
            | IrCmd::STORE_INT
            | IrCmd::STORE_INT64
            | IrCmd::STORE_TVALUE => {
                let source_op = op_b(inst.clone());

                if source_op.kind() == IrOpKind::Inst {
                    let function = unsafe { &mut *self.function };
                    let source = function.instructions[source_op.index() as usize].clone();
                    let can_remat_args =
                        can_rematerialize_arguments_at(function, source_op.index());

                    if source.last_use != inst_idx || can_remat_args {
                        self.record_restore_op(source_op.index(), op_a(inst));
                    }
                }
            }

            IrCmd::STORE_SPLIT_TVALUE => {
                let source_op = op_c(inst.clone());

                if source_op.kind() == IrOpKind::Inst {
                    let function = unsafe { &mut *self.function };
                    let source = function.instructions[source_op.index() as usize].clone();
                    let can_remat_args =
                        can_rematerialize_arguments_at(function, source_op.index());

                    if source.last_use != inst_idx || can_remat_args {
                        self.record_restore_op(source_op.index(), op_a(inst));
                    }
                }
            }

            IrCmd::NUM_TO_UINT | IrCmd::NUM_TO_INT => {
                let arg = op_a(inst);

                if FFlag::LuauCodegenForwardRematerialize.get() && arg.kind() == IrOpKind::Inst {
                    let function = unsafe { &mut *self.function };
                    let owner_loc = function.find_restore_location_u32_bool(arg.index(), true);

                    if owner_loc.op.kind() == IrOpKind::VmReg
                        && owner_loc.kind == IrValueKind::Double
                        && owner_loc.conversion_cmd == IrCmd::NOP
                        && !owner_loc.lazy
                    {
                        let reg = vm_reg_op(owner_loc.op) as usize;
                        let captured =
                            (function.cfg.captured.regs[reg / 64] & (1u64 << (reg % 64))) != 0;

                        if !captured && self.vm_reg_dependent[reg] == k_invalid_inst_idx {
                            let forward_cmd = if inst.cmd == IrCmd::NUM_TO_UINT {
                                IrCmd::UINT_TO_NUM
                            } else {
                                IrCmd::INT_TO_NUM
                            };

                            function.record_restore_location(
                                inst_idx,
                                ValueRestoreLocation {
                                    op: owner_loc.op,
                                    kind: IrValueKind::Double,
                                    conversion_cmd: forward_cmd,
                                    lazy: false,
                                },
                            );

                            self.vm_reg_dependent[reg] = inst_idx;
                        }
                    }
                }
            }

            _ => {}
        }
    }
}

fn can_rematerialize_arguments_at(
    function: &mut crate::records::ir_function::IrFunction,
    inst_idx: u32,
) -> bool {
    let mut inst = function.instructions[inst_idx as usize].clone();

    if (inst.cmd == IrCmd::UINT_TO_NUM || inst.cmd == IrCmd::INT_TO_NUM)
        && op_a(&mut inst).kind() == IrOpKind::Inst
    {
        let dep_inst_idx = op_a(&mut inst).index();

        if function.instructions[dep_inst_idx as usize].last_use != inst_idx {
            return true;
        }
    }

    false
}
