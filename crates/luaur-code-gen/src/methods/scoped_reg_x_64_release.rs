use crate::records::register_x_64::RegisterX64;
use crate::records::scoped_reg_x_64::ScopedRegX64;

impl ScopedRegX64 {
    pub fn release(&mut self) -> RegisterX64 {
        let tmp = self.reg;
        self.reg = RegisterX64::noreg;
        tmp
    }
}
