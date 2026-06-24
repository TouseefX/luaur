use crate::enums::kind_a_64::KindA64;
use crate::functions::countlz_bit_utils::countlz_u32;
use crate::functions::countrz_bit_utils::countrz_u32;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_reg_alloc_a_64::IrRegAllocA64;
use crate::records::register_a_64::RegisterA64;
use crate::records::set::Set;
use luaur_common::FFlag;

impl IrRegAllocA64 {
    pub fn alloc_reg(&mut self, kind: KindA64, index: u32) -> RegisterA64 {
        if FFlag::LuauCodegenVmExitSync.get() {
            self.alloc_action_count += 1;
        }

        let set = self.get_set(kind) as *mut Set;

        if unsafe { (*set).free } == 0 {
            // Try to find and spill a register that is not used in the current instruction and has the furthest next use
            if let Some(furthest_use_target) = unsafe {
                let result = self.find_instruction_with_furthest_next_use(&mut *set);
                if result != Self::kInvalidInstIdx {
                    Some(result)
                } else {
                    None
                }
            } {
                unsafe { self.spill_set_u32_u32(&mut *set, index, furthest_use_target) };
                CODEGEN_ASSERT!(unsafe { (*set).free } != 0);
            } else {
                self.error = true;
                return RegisterA64 {
                    bits: (kind as u8) & RegisterA64::KIND_MASK,
                };
            }
        }

        let mut reg = 31 - countlz_u32(unsafe { (*set).free }) as i32;

        if FFlag::DebugCodegenChaosA64.get() {
            reg = countrz_u32(unsafe { (*set).free }) as i32; // allocate from low end; this causes extra conflicts for calls
        }

        unsafe {
            (*set).free &= !(1u32 << reg);
            (*set).defs[reg as usize] = index;
        }

        RegisterA64 {
            bits: ((kind as u8) & RegisterA64::KIND_MASK)
                | ((reg as u8) << RegisterA64::INDEX_SHIFT),
        }
    }
}
