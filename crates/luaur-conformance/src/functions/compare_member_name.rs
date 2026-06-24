pub fn compare_member_name(
    member: *const core::ffi::c_char,
    member_length: usize,
    str: *const core::ffi::c_char,
) -> bool {
    let str_bytes = unsafe { core::ffi::CStr::from_ptr(str) }.to_bytes();
    if member_length != str_bytes.len() {
        return false;
    }

    let member_bytes = unsafe { core::slice::from_raw_parts(member as *const u8, member_length) };
    member_bytes == str_bytes
}
