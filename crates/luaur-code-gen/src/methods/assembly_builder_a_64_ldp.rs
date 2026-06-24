use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn ldp(&mut self, dst1: RegisterA64, dst2: RegisterA64, src: AddressA64) {
        debug_assert!(dst1.kind() == KindA64::x || dst1.kind() == KindA64::w);
        debug_assert!(dst1.kind() == dst2.kind());

        let is_x = dst1.kind() == KindA64::x;
        self.place_p(
            b"ldp\0".as_ptr() as *const core::ffi::c_char,
            dst1,
            dst2,
            src,
            0b101_0_010_1,
            (is_x as u8) << 1,
            if is_x { 3 } else { 2 },
        );
    }
}
