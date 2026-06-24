use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn fcmp(&mut self, src1: RegisterA64, src2: RegisterA64) {
        debug_assert!(src1.kind() == src2.kind());
        debug_assert!(src1.kind() == KindA64::d || src1.kind() == KindA64::s);

        if src1.kind() == KindA64::d {
            self.assembly_builder_a_64_place_fcmp(c"fcmp".as_ptr(), src1, src2, 0b11110_01_1, 0b00);
        } else {
            self.assembly_builder_a_64_place_fcmp(c"fcmp".as_ptr(), src1, src2, 0b11110_00_1, 0b00);
        }
    }
}
