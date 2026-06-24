use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn fcmpz(&mut self, src: RegisterA64) {
        CODEGEN_ASSERT!(src.kind() == KindA64::d || src.kind() == KindA64::s);

        let zero_reg = RegisterA64 {
            bits: src.kind() as u8,
        };

        if src.kind() == KindA64::d {
            self.assembly_builder_a_64_place_fcmp(
                c"fcmp".as_ptr(),
                src,
                zero_reg,
                0b11110_01_1,
                0b01,
            );
        } else {
            self.assembly_builder_a_64_place_fcmp(
                c"fcmp".as_ptr(),
                src,
                zero_reg,
                0b11110_00_1,
                0b01,
            );
        }
    }
}
