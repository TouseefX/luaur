use crate::enums::arch::Arch;
use crate::records::register_a_64::RegisterA64;
use crate::records::register_x_64::RegisterX64;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UnwindBuilder {
    _vtable: *const core::ffi::c_void,
}

impl UnwindBuilder {
    #[allow(non_upper_case_globals)]
    pub const X64: Arch = Arch::X64;
    #[allow(non_upper_case_globals)]
    pub const A64: Arch = Arch::A64;
}

#[allow(non_camel_case_types)]
pub type UnwindBuilder_Arch = Arch;

impl Default for UnwindBuilder {
    fn default() -> Self {
        Self {
            _vtable: core::ptr::null(),
        }
    }
}
