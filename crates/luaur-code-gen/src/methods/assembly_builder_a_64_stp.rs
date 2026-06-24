use crate::enums::kind_a_64::KindA64;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn stp(&mut self, src1: RegisterA64, src2: RegisterA64, dst: AddressA64) {
        debug_assert!(src1.kind() == KindA64::x || src1.kind() == KindA64::w);
        debug_assert!(src1.kind() == src2.kind());

        let is_x = src1.kind() == KindA64::x;
        self.place_p(
            b"stp\0".as_ptr() as *const core::ffi::c_char,
            src1,
            src2,
            dst,
            0b101_0_010_0,
            (is_x as u8) << 1,
            if is_x { 3 } else { 2 },
        );
    }
}
