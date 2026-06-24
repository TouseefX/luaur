use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::register_x_64::RegisterX64;
use crate::records::scoped_reg_x_64::ScopedRegX64;

impl ScopedRegX64 {
    pub fn scoped_reg_x_64_scoped_reg_x_64(&mut self) {
        if self.reg != RegisterX64::noreg {
            unsafe { &mut *self.owner }.free_reg(self.reg);
        }
    }
}
