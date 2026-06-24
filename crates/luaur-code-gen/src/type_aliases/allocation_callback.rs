#[allow(non_camel_case_types)]
pub type AllocationCallback = unsafe extern "C" fn(
    context: *mut core::ffi::c_void,
    old_pointer: *mut core::ffi::c_void,
    old_size: usize,
    new_pointer: *mut core::ffi::c_void,
    new_size: usize,
);
