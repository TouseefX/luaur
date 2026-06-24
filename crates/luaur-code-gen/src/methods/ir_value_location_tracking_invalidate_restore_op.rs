use crate::enums::ir_op_kind::IrOpKind;
use crate::enums::ir_value_kind::IrValueKind;
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
    pub fn invalidate_restore_op(&mut self, location: IrOp, skip_value_invalidation: bool) {
        if location.kind() == IrOpKind::VmReg {
            let reg = vm_reg_op(location) as usize;
            let inst_idx = self.vm_reg_value[reg];

            if inst_idx != k_invalid_inst_idx {
                self.invalidate_inst_restore_location(inst_idx, location, skip_value_invalidation);
                self.vm_reg_value[reg] = k_invalid_inst_idx;

                if FFlag::LuauCodegenForwardRematerialize.get() {
                    let dep_inst_idx = self.vm_reg_dependent[reg];

                    if dep_inst_idx != k_invalid_inst_idx {
                        self.invalidate_inst_restore_location(dep_inst_idx, location, false);
                        self.vm_reg_dependent[reg] = k_invalid_inst_idx;
                    }
                } else {
                    let function = unsafe { &mut *self.function };
                    let inst = function.instructions[inst_idx as usize].clone();
                    let mut inst_for_op = inst.clone();

                    if self.can_be_rematerialized(inst.cmd)
                        && op_a(&mut inst_for_op).kind() == IrOpKind::Inst
                    {
                        let dep_inst_idx = op_a(&mut inst_for_op).index();

                        if function.instructions[dep_inst_idx as usize].needs_reload {
                            if let Some(callback) = self.restore_callback {
                                unsafe {
                                    callback(
                                        self.restore_callback_ctx,
                                        &mut function.instructions[dep_inst_idx as usize],
                                    );
                                }
                            }
                        }

                        let curr_restore_location =
                            function.find_restore_location_u32_bool(inst_idx, false);

                        if location == curr_restore_location.op {
                            function.record_restore_location(
                                dep_inst_idx,
                                ValueRestoreLocation::default(),
                            );
                        }
                    }
                }
            }
        } else if location.kind() == IrOpKind::VmConst {
            CODEGEN_ASSERT!(false);
        }
    }

    fn invalidate_inst_restore_location(
        &mut self,
        inst_idx: u32,
        location: IrOp,
        skip_value_invalidation: bool,
    ) {
        let function = unsafe { &mut *self.function };
        let inst_cmd = function.instructions[inst_idx as usize].cmd;
        let needs_reload = function.instructions[inst_idx as usize].needs_reload;

        if skip_value_invalidation {
            match get_cmd_value_kind(inst_cmd) {
                IrValueKind::Double
                | IrValueKind::Pointer
                | IrValueKind::Int
                | IrValueKind::Int64 => {
                    return;
                }
                _ => {}
            }
        }

        if needs_reload {
            CODEGEN_ASSERT!(
                !function
                    .find_restore_location_u32_bool(inst_idx, false)
                    .lazy
            );

            if let Some(callback) = self.restore_callback {
                unsafe {
                    let inst = &mut function.instructions[inst_idx as usize];
                    callback(self.restore_callback_ctx, inst);
                }
            }
        }

        let curr_restore_location = function.find_restore_location_u32_bool(inst_idx, false);

        if location == curr_restore_location.op {
            function.record_restore_location(inst_idx, ValueRestoreLocation::default());
        }
    }
}
