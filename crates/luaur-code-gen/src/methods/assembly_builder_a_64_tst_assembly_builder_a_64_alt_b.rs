use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn tst_register_a_64_u32(&mut self, src1: RegisterA64, src2: u32) {
        let dst = if src1.kind() == KindA64::x {
            RegisterA64::xzr
        } else {
            RegisterA64::wzr
        };

        self.place_bm(c"tst".as_ptr(), dst, src1, src2, 0b11_100100);
    }
}
