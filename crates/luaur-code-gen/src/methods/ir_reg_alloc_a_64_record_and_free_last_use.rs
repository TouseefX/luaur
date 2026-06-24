use crate::functions::free_spill::free_spill;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::exit_sync_arg_a_64::ExitSyncArgA64;
use crate::records::ir_inst::IrInst;
use crate::records::ir_reg_alloc_a_64::IrRegAllocA64;
use crate::records::register_a_64::RegisterA64;
use crate::records::value_restore_location::ValueRestoreLocation;

const K_NO_SPILL_SLOT: i8 = -1;

impl IrRegAllocA64 {
    pub fn record_and_free_last_use(
        &mut self,
        block_idx: u32,
        target: &mut IrInst,
        origin_inst_idx: u32,
    ) {
        let mut arg = ExitSyncArgA64 {
            inst_idx: unsafe { &*self.function }.get_inst_index(target),
            reg: RegisterA64::noreg,
            slot: 0,
            original_reg: RegisterA64::noreg,
            restore_location: ValueRestoreLocation::default(),
        };

        if target.spilled || target.needs_reload {
            let mut i = 0;
            while i < self.spills.len() {
                if self.spills[i].inst == arg.inst_idx {
                    let spill = self.spills[i].clone();

                    arg.original_reg = spill.origin;
                    arg.slot = spill.slot;

                    // Capture restore location state at the current instruction
                    if arg.slot == K_NO_SPILL_SLOT {
                        arg.restore_location = unsafe { &*self.function }
                            .find_restore_location_ir_inst_bool(target, false);
                    }

                    // If this was the last use, free register by not restoring it fully and remove the spill record
                    if target.last_use == origin_inst_idx && !target.reused_reg {
                        if arg.slot >= 0 {
                            free_spill(
                                &mut self.free_spill_slots,
                                spill.origin.kind(),
                                spill.slot as u8,
                            );
                        }

                        CODEGEN_ASSERT!(target.reg_a64 == RegisterA64::noreg);
                        target.spilled = false;
                        target.needs_reload = false;

                        self.spills[i] = self.spills[self.spills.len() - 1].clone();
                        self.spills.pop();
                    }

                    break;
                }
                i += 1;
            }
        } else {
            CODEGEN_ASSERT!(target.reg_a64 != RegisterA64::noreg);
            arg.reg = target.reg_a64;
            arg.original_reg = target.reg_a64;

            if target.last_use == origin_inst_idx && !target.reused_reg {
                self.free_reg(target.reg_a64);
                target.reg_a64 = RegisterA64::noreg;
            }
        }

        let entry = self.exit_sync_args.get_or_insert(block_idx);
        entry.push(arg);
    }
}
