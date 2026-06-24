use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn cmp_register_a_64_register_a_64(&mut self, src1: RegisterA64, src2: RegisterA64) {
        let xzr = RegisterA64::xzr;
        let wzr = RegisterA64::wzr;

        let dst = if src1.kind() == KindA64::x { xzr } else { wzr };

        self.place_sr_3(c"cmp".as_ptr(), dst, src1, src2, 0b11_01011, 0, 0);
    }
}
