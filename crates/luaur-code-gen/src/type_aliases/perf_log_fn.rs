#[allow(non_camel_case_types)]
pub type PerfLogFn = Option<
    unsafe extern "C" fn(
        context: *mut core::ffi::c_void,
        addr: usize,
        size: core::ffi::c_uint,
        symbol: *const core::ffi::c_char,
    ),
>;
