use crate::enums::kind_a_64::KindA64;
use crate::functions::countlz_bit_utils::countlz_u32;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_reg_alloc_a_64::IrRegAllocA64;
use crate::records::register_a_64::RegisterA64;
use crate::records::set::Set;
use luaur_common::FFlag;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
    RegisterA64 {
        bits: kind as u8 | (index << RegisterA64::INDEX_SHIFT),
    }
}

impl IrRegAllocA64 {
    pub fn spill_u32_initializer_list_register_a_64(
        &mut self,
        index: u32,
        live: &[RegisterA64],
    ) -> usize {
        let sets = [KindA64::x, KindA64::q];

        let start = self.spills.len();

        let mut poisongpr = 0u32;
        let mut poisonsimd = 0u32;

        if FFlag::DebugCodegenChaosA64.get() {
            poisongpr = self.gpr.base & !self.gpr.free;
            poisonsimd = self.simd.base & !self.simd.free;

            for reg in live.iter() {
                if matches!(reg.kind(), KindA64::s | KindA64::d | KindA64::q) {
                    poisonsimd &= !(1u32 << reg.index());
                } else {
                    poisongpr &= !(1u32 << reg.index());
                }
            }
        }

        for kind in sets.iter() {
            let set = self.get_set(*kind) as *mut Set;

            // early-out
            if unsafe { (*set).free == (*set).base } {
                continue;
            }

            // free all temp registers
            CODEGEN_ASSERT!(unsafe { ((*set).free & (*set).temp) == 0 });
            unsafe {
                (*set).free |= (*set).temp;
                (*set).temp = 0;
            }

            // spill all allocated registers unless they aren't used anymore
            let mut regs = unsafe { (*set).base & !(*set).free };

            while regs != 0 {
                let reg = 31 - countlz_u32(regs) as i32;

                let target_inst_idx = unsafe { (*set).defs[reg as usize] };

                CODEGEN_ASSERT!(target_inst_idx != IrRegAllocA64::kInvalidInstIdx);
                CODEGEN_ASSERT!(
                    unsafe { &mut *self.function }.instructions[target_inst_idx as usize]
                        .reg_a64
                        .index()
                        == reg as u8
                );

                unsafe { self.spill_set_u32_u32(&mut *set, index, target_inst_idx) };

                regs &= !(1u32 << reg as u32);
            }

            CODEGEN_ASSERT!(unsafe { (*set).free == (*set).base });
        }

        if FFlag::DebugCodegenChaosA64.get() {
            for i in 0..32 {
                if poisongpr & (1u32 << i) != 0 {
                    unsafe { &mut *self.build }
                        .mov_register_a_64_i32(reg(KindA64::x, i as u8), 0xdead);
                }
                if poisonsimd & (1u32 << i) != 0 {
                    unsafe { &mut *self.build }
                        .fmov_register_a_64_f64(reg(KindA64::d, i as u8), -0.125);
                }
            }
        }

        start
    }
}
