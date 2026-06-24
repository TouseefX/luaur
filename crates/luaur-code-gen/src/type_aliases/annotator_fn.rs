#[allow(non_camel_case_types)]
pub type AnnotatorFn = Option<
    unsafe extern "C" fn(
        context: *mut core::ffi::c_void,
        result: &mut alloc::string::String,
        fid: core::ffi::c_int,
        instpos: core::ffi::c_int,
    ),
>;
