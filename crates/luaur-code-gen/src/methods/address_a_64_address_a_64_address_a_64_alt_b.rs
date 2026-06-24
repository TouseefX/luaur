use crate::enums::address_kind_a_64::AddressKindA64;
use crate::enums::kind_a_64::KindA64;
use crate::records::address_a_64::AddressA64;
use crate::records::register_a_64::RegisterA64;

impl AddressA64 {
    pub fn address_a_64_register_a_64_register_a_64(
        base: RegisterA64,
        offset: RegisterA64,
    ) -> Self {
        debug_assert!(base.kind() == KindA64::x);
        debug_assert!(offset.kind() == KindA64::x);

        Self {
            kind: AddressKindA64::reg,
            base,
            offset,
            data: 0,
        }
    }
}
