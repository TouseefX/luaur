use crate::functions::compare_member_name::compare_member_name;

#[allow(non_upper_case_globals)]
const LBC_TYPE_ANY: u8 = 15;
#[allow(non_upper_case_globals)]
const LBC_TYPE_NUMBER: u8 = 2;
#[allow(non_upper_case_globals)]
const LBC_TYPE_VECTOR: u8 = 8;

pub fn vector_namecall_bytecode_type(member: *const core::ffi::c_char, member_length: usize) -> u8 {
    if compare_member_name(member, member_length, c"Dot".as_ptr()) {
        return LBC_TYPE_NUMBER;
    }

    if compare_member_name(member, member_length, c"Cross".as_ptr()) {
        return LBC_TYPE_VECTOR;
    }

    LBC_TYPE_ANY
}
