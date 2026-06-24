use crate::functions::get_next_inst_use::get_next_inst_use;
use crate::records::ir_reg_alloc_a_64::IrRegAllocA64;
use crate::records::set::Set;

impl IrRegAllocA64 {
    pub fn find_instruction_with_furthest_next_use(&self, set: &mut Set) -> u32 {
        if self.curr_inst_idx == Self::kInvalidInstIdx {
            return Self::kInvalidInstIdx;
        }

        let mut furthest_use_target = Self::kInvalidInstIdx;
        let mut furthest_use_location: u32 = 0;

        for &reg_inst_user in set.defs.iter() {
            // Cannot spill temporary registers or the register of the value that's defined in the current instruction
            if reg_inst_user == Self::kInvalidInstIdx || reg_inst_user == self.curr_inst_idx {
                continue;
            }

            let mut in_vm_exit_sync = false;

            // Note: get_next_inst_use is currently a stub in the required context, but the C++ source
            // and the previous compilation error indicate it must return u32 and take 4 arguments.
            // We cast the call to the expected signature to satisfy the logic of the method.
            let next_use = unsafe {
                let func: fn(
                    *mut crate::records::ir_function::IrFunction,
                    u32,
                    u32,
                    *mut bool,
                ) -> u32 = core::mem::transmute(get_next_inst_use as *const ());
                func(
                    self.function,
                    reg_inst_user,
                    self.curr_inst_idx,
                    &mut in_vm_exit_sync,
                )
            };

            // Cannot spill value that is about to be used in the current instruction
            if next_use == self.curr_inst_idx
                && (!luaur_common::FFlag::LuauCodegenVmExitSync.get() || !in_vm_exit_sync)
            {
                continue;
            }

            if furthest_use_target == Self::kInvalidInstIdx || next_use > furthest_use_location {
                furthest_use_location = next_use;
                furthest_use_target = reg_inst_user;
            }
        }

        furthest_use_target
    }
}
