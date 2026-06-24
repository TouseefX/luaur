use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn fneg(&mut self, dst: RegisterA64, src: RegisterA64) {
        if dst.kind() == KindA64::d {
            debug_assert!(src.kind() == KindA64::d);

            self.place_r_1(
                b"fneg\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b000_11110_01_1_0000_10_10000,
            );
        } else if dst.kind() == KindA64::s {
            debug_assert!(src.kind() == KindA64::s);

            self.place_r_1(
                b"fneg\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b000_11110_00_1_0000_10_10000,
            );
        } else {
            debug_assert!(dst.kind() == KindA64::q && src.kind() == KindA64::q);

            self.place_r_1(
                b"fneg\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b011_01110_1_0_10000_01111_10,
            );
        }
    }
}
