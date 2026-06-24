pub type UserdataRemapperCallback = unsafe extern "C" fn(
    context: *mut core::ffi::c_void,
    name: *const core::ffi::c_char,
    name_length: usize,
) -> u8;
