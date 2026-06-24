use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn scvtf(&mut self, dst: RegisterA64, src: RegisterA64) {
        debug_assert!(dst.kind() == KindA64::d);
        debug_assert!(src.kind() == KindA64::w || src.kind() == KindA64::x);

        self.place_r_1(
            b"scvtf\0".as_ptr() as *const core::ffi::c_char,
            dst,
            src,
            0b000_11110_01_1_00_010_000000,
        );
    }
}
