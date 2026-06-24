use crate::functions::compare_member_name::compare_member_name;

const LBC_TYPE_NUMBER: u8 = 2;
const LBC_TYPE_VECTOR: u8 = 8;
const LBC_TYPE_ANY: u8 = 15;

pub fn vector_access_bytecode_type(member: *const core::ffi::c_char, member_length: usize) -> u8 {
    if compare_member_name(member, member_length, c"Magnitude".as_ptr()) {
        return LBC_TYPE_NUMBER;
    }

    if compare_member_name(member, member_length, c"Unit".as_ptr()) {
        return LBC_TYPE_VECTOR;
    }

    LBC_TYPE_ANY
}
