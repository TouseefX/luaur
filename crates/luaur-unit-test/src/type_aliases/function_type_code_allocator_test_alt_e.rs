pub type FunctionType = extern "C" fn(
    *mut core::ffi::c_void,
    Option<extern "C" fn(i64)>,
    *mut core::ffi::c_void,
) -> i64;
