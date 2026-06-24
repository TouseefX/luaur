use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn fcvtzu(&mut self, dst: RegisterA64, src: RegisterA64) {
        debug_assert!(dst.kind() == KindA64::w || dst.kind() == KindA64::x);
        debug_assert!(src.kind() == KindA64::d);

        self.place_r_1(
            b"fcvtzu\0".as_ptr() as *const core::ffi::c_char,
            dst,
            src,
            0b000_11110_01_1_11_001_000000,
        );
    }
}
