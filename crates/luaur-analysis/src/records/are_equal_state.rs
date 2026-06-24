use std::collections::BTreeSet;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AreEqualState {
    pub(crate) seen: BTreeSet<(*const core::ffi::c_void, *const core::ffi::c_void)>,
    pub(crate) recursion_count: i32,
}

unsafe impl Send for AreEqualState {}
unsafe impl Sync for AreEqualState {}
