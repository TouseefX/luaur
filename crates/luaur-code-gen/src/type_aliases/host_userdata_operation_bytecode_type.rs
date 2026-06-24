#[allow(non_camel_case_types)]
pub type HostUserdataOperationBytecodeType = Option<
    unsafe extern "C" fn(r#type: u8, member: *const core::ffi::c_char, member_length: usize) -> u8,
>;
