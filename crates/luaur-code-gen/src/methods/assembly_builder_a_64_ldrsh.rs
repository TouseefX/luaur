use crate::enums::kind_a_64::KindA64;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn ldrsh(&mut self, dst: RegisterA64, src: AddressA64) {
        debug_assert!(dst.kind() == KindA64::x || dst.kind() == KindA64::w);

        let opsize = if dst.kind() == KindA64::w {
            0b01_1110_0010 | 0b01
        } else {
            0b01_1110_0010 | 0b00
        };

        self.place_a(
            b"ldrsh\0".as_ptr() as *const core::ffi::c_char,
            dst,
            src,
            opsize as u16,
            1,
        );
    }
}
