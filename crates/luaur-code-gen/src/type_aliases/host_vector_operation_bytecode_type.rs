#[allow(non_camel_case_types)]
pub type HostVectorOperationBytecodeType =
    Option<unsafe extern "C" fn(member: *const core::ffi::c_char, member_length: usize) -> u8>;
