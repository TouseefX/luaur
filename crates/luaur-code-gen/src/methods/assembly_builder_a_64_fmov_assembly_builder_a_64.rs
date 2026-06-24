use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn fmov_register_a_64_register_a_64(&mut self, dst: RegisterA64, src: RegisterA64) {
        if dst.kind() == KindA64::d && src.kind() == KindA64::d {
            self.place_r_1(
                b"fmov\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b00_11110_01_1_0000_00_10000,
            );
        } else if dst.kind() == KindA64::d && src.kind() == KindA64::x {
            self.place_r_1(
                b"fmov\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b00_11110_01_1_00_111_000000,
            );
        } else if dst.kind() == KindA64::x && src.kind() == KindA64::d {
            self.place_r_1(
                b"fmov\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b00_11110_01_1_00_110_000000,
            );
        } else if dst.kind() == KindA64::s && src.kind() == KindA64::s {
            self.place_r_1(
                b"fmov\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b00_11110_00_1_0000_00_10000,
            );
        } else if dst.kind() == KindA64::s && src.kind() == KindA64::w {
            self.place_r_1(
                b"fmov\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b00_11110_00_1_00_111_000000,
            );
        } else {
            panic!("Unsupported fmov kind");
        }
    }
}
