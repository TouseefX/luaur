use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn cmn(&mut self, src1: RegisterA64, src2: u16) {
        let dst = if src1.kind() == crate::enums::kind_a_64::KindA64::x {
            RegisterA64::xzr
        } else {
            RegisterA64::wzr
        };

        self.place_i12(c"cmn".as_ptr(), dst, src1, src2 as i32, 0b01_10001);
    }
}
