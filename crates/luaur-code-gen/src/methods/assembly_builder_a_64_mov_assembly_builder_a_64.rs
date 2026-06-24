use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn mov_register_a_64_register_a_64(&mut self, dst: RegisterA64, src: RegisterA64) {
        if dst.kind() != KindA64::q {
            debug_assert!(
                dst.kind() == KindA64::w || dst.kind() == KindA64::x || dst == RegisterA64::sp
            );
            debug_assert!(
                dst.kind() == src.kind()
                    || (dst.kind() == KindA64::x && src == RegisterA64::sp)
                    || (dst == RegisterA64::sp && src.kind() == KindA64::x)
            );

            if dst == RegisterA64::sp || src == RegisterA64::sp {
                self.place_r_1(
                    b"mov\0".as_ptr() as *const core::ffi::c_char,
                    dst,
                    src,
                    0b00_100010_0_000000000000,
                );
            } else {
                self.place_sr_2(
                    b"mov\0".as_ptr() as *const core::ffi::c_char,
                    dst,
                    src,
                    0b01_01010,
                    0,
                );
            }
        } else {
            debug_assert!(dst.kind() == src.kind());

            self.place_r_1(
                b"mov\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b10_01110_10_1_00000_00011_1 | ((src.index() as u32) << 6),
            );
        }
    }
}
