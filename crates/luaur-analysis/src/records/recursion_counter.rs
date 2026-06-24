#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct RecursionCounter {
    pub(crate) count: *mut core::ffi::c_int,
}
