use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::emit_common_x_64;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;

impl IrRegAllocX64 {
    pub fn is_extra_spill_slot(&self, slot: u32) -> bool {
        CODEGEN_ASSERT!(slot != emit_common_x_64::K_NO_STACK_SLOT);
        slot >= emit_common_x_64::K_SPILL_SLOTS * 2
    }
}
