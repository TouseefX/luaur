use crate::enums::kind_a_64::KindA64;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn strh(&mut self, src: RegisterA64, dst: AddressA64) {
        debug_assert!(src.kind() == KindA64::w);

        self.place_a(
            b"strh\0".as_ptr() as *const core::ffi::c_char,
            src,
            dst,
            0b01_11100000,
            1,
        );
    }
}
