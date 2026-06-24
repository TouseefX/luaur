use crate::enums::kind_a_64::KindA64;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn str(&mut self, src: RegisterA64, dst: AddressA64) {
        assert!(
            src.kind() == KindA64::x
                || src.kind() == KindA64::w
                || src.kind() == KindA64::s
                || src.kind() == KindA64::d
                || src.kind() == KindA64::q
        );

        match src.kind() {
            KindA64::w => self.place_a(
                b"str\0".as_ptr() as *const core::ffi::c_char,
                src,
                dst,
                0b10_11100000,
                2,
            ),
            KindA64::x => self.place_a(
                b"str\0".as_ptr() as *const core::ffi::c_char,
                src,
                dst,
                0b11_11100000,
                3,
            ),
            KindA64::s => self.place_a(
                b"str\0".as_ptr() as *const core::ffi::c_char,
                src,
                dst,
                0b10_11110000,
                2,
            ),
            KindA64::d => self.place_a(
                b"str\0".as_ptr() as *const core::ffi::c_char,
                src,
                dst,
                0b11_11110000,
                3,
            ),
            KindA64::q => self.place_a(
                b"str\0".as_ptr() as *const core::ffi::c_char,
                src,
                dst,
                0b00_11110010,
                4,
            ),
            KindA64::none => unreachable!("Unexpected register kind"),
        }
    }
}
