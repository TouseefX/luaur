use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn frinta(&mut self, dst: RegisterA64, src: RegisterA64) {
        debug_assert!(dst.kind() == src.kind());
        debug_assert!(
            dst.kind() == KindA64::d || dst.kind() == KindA64::s || dst.kind() == KindA64::q
        );

        if dst.kind() == KindA64::q {
            self.place_r_1(
                b"frinta\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b011_01110_00_1_0000_11000_10,
            );
        } else if dst.kind() == KindA64::d {
            self.place_r_1(
                b"frinta\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b000_11110_01_1_001_100_10000,
            );
        } else {
            self.place_r_1(
                b"frinta\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b000_11110_00_1_001_100_10000,
            );
        }
    }
}
