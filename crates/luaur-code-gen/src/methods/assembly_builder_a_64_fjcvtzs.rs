use crate::enums::features_a_64::FeaturesA64;
use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn fjcvtzs(&mut self, dst: RegisterA64, src: RegisterA64) {
        debug_assert!(dst.kind() == KindA64::w);
        debug_assert!(src.kind() == KindA64::d);
        debug_assert!(self.features & FeaturesA64::Feature_JSCVT as u32 != 0);

        self.place_r_1(
            b"fjcvtzs\0".as_ptr() as *const core::ffi::c_char,
            dst,
            src,
            0b000_11110_01_1_11_110_000000,
        );
    }
}
