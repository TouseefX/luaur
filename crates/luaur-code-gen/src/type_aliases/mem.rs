use crate::enums::address_kind_a_64::AddressKindA64;
use crate::records::address_a_64::AddressA64;
use crate::records::register_a_64::RegisterA64;

#[allow(non_camel_case_types)]
pub type mem = AddressA64;

/// The C++ `using mem = AddressA64` is constructed as `mem(base, off)` /
/// `mem(base, regoffset)` (AddressA64's overloaded constructor). A Rust type
/// alias isn't callable, so this free `mem(...)` reproduces the two-argument
/// forms (offset kind defaults to `imm`, as in the C++ ctor). The 3-argument
/// `mem(base, off, kind)` form maps directly to the `AddressA64` constructor.
pub trait MemOffset {
    fn into_mem(self, base: RegisterA64) -> AddressA64;
}

impl MemOffset for i32 {
    fn into_mem(self, base: RegisterA64) -> AddressA64 {
        AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
            base,
            self,
            AddressKindA64::imm,
        )
    }
}

impl MemOffset for RegisterA64 {
    fn into_mem(self, base: RegisterA64) -> AddressA64 {
        AddressA64::address_a_64_register_a_64_register_a_64(base, self)
    }
}

#[allow(non_snake_case)]
pub fn mem<O: MemOffset>(base: RegisterA64, off: O) -> AddressA64 {
    off.into_mem(base)
}
