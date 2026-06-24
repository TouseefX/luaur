use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;
use crate::records::register_x_64::RegisterX64;

impl IrRegAllocX64 {
    pub fn assert_free(&self, reg: RegisterX64) {
        if reg.size() == crate::enums::size_x_64::SizeX64::xmmword {
            CODEGEN_ASSERT!(self.free_xmm_map[reg.index() as usize]);
        } else {
            CODEGEN_ASSERT!(self.free_gpr_map[reg.index() as usize]);
        }
    }
}
