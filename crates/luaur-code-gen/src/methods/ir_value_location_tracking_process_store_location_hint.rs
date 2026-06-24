use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_data::k_invalid_inst_idx;
use crate::records::ir_value_location_tracking::IrValueLocationTracking;
use crate::records::store_location_hint::StoreLocationHint;
use crate::records::value_restore_location::ValueRestoreLocation;

impl IrValueLocationTracking {
    pub fn process_store_location_hint(&mut self, hint: &StoreLocationHint) {
        CODEGEN_ASSERT!(hint.op.kind() == IrOpKind::VmReg);

        if hint.inst_idx != k_invalid_inst_idx {
            let function = unsafe { &mut *self.function };

            if function.instructions[hint.inst_idx as usize].use_count == 0 {
                return;
            }

            let reg = vm_reg_op(hint.op);
            let existing_loc = function.find_restore_location_u32_bool(hint.inst_idx, false);

            if existing_loc.op.kind() != IrOpKind::None {
                return;
            }

            if reg > self.max_reg {
                self.max_reg = reg;
            }

            let captured = (function.cfg.captured.regs[reg as usize / 64]
                & (1u64 << (reg as usize % 64)))
                != 0;

            self.invalidate_restore_op(hint.op, false);

            if !captured {
                function.record_restore_location(
                    hint.inst_idx,
                    ValueRestoreLocation {
                        op: hint.op,
                        kind: hint.kind,
                        conversion_cmd: crate::enums::ir_cmd::IrCmd::NOP,
                        lazy: true,
                    },
                );
            }

            self.vm_reg_value[reg as usize] = hint.inst_idx;
        }
    }
}
