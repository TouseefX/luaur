use crate::enums::kind_a_64::KindA64;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;
use luaur_common::macros::luau_unreachable::LUAU_UNREACHABLE;

impl AssemblyBuilderA64 {
    pub fn ldr(&mut self, dst: RegisterA64, src: AddressA64) {
        match dst.kind() {
            KindA64::w => self.place_a(
                b"ldr\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b10_1110_0001,
                2,
            ),
            KindA64::x => self.place_a(
                b"ldr\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b11_1110_0001,
                3,
            ),
            KindA64::s => self.place_a(
                b"ldr\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b10_1111_0001,
                2,
            ),
            KindA64::d => self.place_a(
                b"ldr\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b11_1111_0001,
                3,
            ),
            KindA64::q => self.place_a(
                b"ldr\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b00_1111_0011,
                4,
            ),
            KindA64::none => {
                debug_assert!(false, "Unexpected register kind");
                LUAU_UNREACHABLE!();
            }
        }
    }
}
