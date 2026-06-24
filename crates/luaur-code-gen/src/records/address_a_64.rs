use crate::enums::address_kind_a_64::AddressKindA64;
use crate::records::register_a_64::RegisterA64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct AddressA64 {
    pub kind: AddressKindA64,
    pub base: RegisterA64,
    pub offset: RegisterA64,
    pub data: core::ffi::c_int,
}

impl AddressA64 {
    pub const kMaxOffset: usize = 1023;
}
