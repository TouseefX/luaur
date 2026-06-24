use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn cmp_register_a_64_u16(&mut self, src1: RegisterA64, src2: u16) {
        let xzr = RegisterA64::xzr;
        let wzr = RegisterA64::wzr;

        let dst = if src1.kind() == KindA64::x { xzr } else { wzr };

        self.place_i12(c"cmp".as_ptr(), dst, src1, src2 as i32, 0b11_10001);
    }
}
