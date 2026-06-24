use crate::enums::address_kind_a_64::AddressKindA64;
use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::address_a_64::AddressA64;
use crate::records::register_a_64::RegisterA64;

impl AddressA64 {
    pub fn address_a_64_register_a_64_i32_address_kind_a_64(
        base: RegisterA64,
        off: core::ffi::c_int,
        kind: AddressKindA64,
    ) -> Self {
        CODEGEN_ASSERT!(base.kind() == KindA64::x || base == RegisterA64::sp);
        CODEGEN_ASSERT!(kind != AddressKindA64::reg);

        Self {
            kind,
            base,
            offset: RegisterA64::noreg,
            data: off,
        }
    }
}
