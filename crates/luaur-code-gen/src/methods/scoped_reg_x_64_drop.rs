use crate::records::register_x_64::RegisterX64;
use crate::records::scoped_reg_x_64::ScopedRegX64;

impl Drop for ScopedRegX64 {
    fn drop(&mut self) {
        if self.reg != RegisterX64::noreg {
            unsafe { &mut *self.owner }.free_reg(self.reg);
        }
    }
}
