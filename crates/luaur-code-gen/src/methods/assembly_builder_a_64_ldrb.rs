use crate::enums::kind_a_64::KindA64;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn ldrb(&mut self, dst: RegisterA64, src: AddressA64) {
        debug_assert!(dst.kind() == KindA64::w);

        self.place_a(
            b"ldrb\0".as_ptr() as *const core::ffi::c_char,
            dst,
            src,
            0b00_1110_0001,
            0,
        );
    }
}
