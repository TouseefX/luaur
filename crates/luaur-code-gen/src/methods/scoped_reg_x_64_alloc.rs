use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_data::k_invalid_inst_idx;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;
use crate::records::register_x_64::RegisterX64;
use crate::records::scoped_reg_x_64::ScopedRegX64;

impl ScopedRegX64 {
    pub fn alloc(&mut self, size: SizeX64) {
        CODEGEN_ASSERT!(self.reg == RegisterX64::noreg);
        let owner = unsafe { &mut *self.owner };
        self.reg = owner.alloc_reg(size, k_invalid_inst_idx);
    }
}
