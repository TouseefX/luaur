use crate::functions::compare_member_name::compare_member_name;
use crate::functions::type_to_userdata_index::type_to_userdata_index;
use crate::functions::userdata_index_to_type::userdata_index_to_type;

#[allow(non_upper_case_globals)]
const LBC_TYPE_ANY: u8 = 0;
#[allow(non_upper_case_globals)]
const LBC_TYPE_NUMBER: u8 = 2;
#[allow(non_upper_case_globals)]
const LBC_TYPE_VECTOR: u8 = 8;

const K_USERDATA_COLOR: u8 = 1;
const K_USERDATA_VEC2: u8 = 2;
const K_USERDATA_MAT3: u8 = 3;
const K_USERDATA_VERTEX: u8 = 4;

pub fn userdata_access_bytecode_type(
    r#type: u8,
    member: *const core::ffi::c_char,
    member_length: usize,
) -> u8 {
    match type_to_userdata_index(r#type) {
        K_USERDATA_COLOR => {
            if compare_member_name(member, member_length, c"R".as_ptr()) {
                return LBC_TYPE_NUMBER;
            }
            if compare_member_name(member, member_length, c"G".as_ptr()) {
                return LBC_TYPE_NUMBER;
            }
            if compare_member_name(member, member_length, c"B".as_ptr()) {
                return LBC_TYPE_NUMBER;
            }
        }
        K_USERDATA_VEC2 => {
            if compare_member_name(member, member_length, c"X".as_ptr()) {
                return LBC_TYPE_NUMBER;
            }
            if compare_member_name(member, member_length, c"Y".as_ptr()) {
                return LBC_TYPE_NUMBER;
            }
            if compare_member_name(member, member_length, c"Magnitude".as_ptr()) {
                return LBC_TYPE_NUMBER;
            }
            if compare_member_name(member, member_length, c"Unit".as_ptr()) {
                return userdata_index_to_type(K_USERDATA_VEC2);
            }
        }
        K_USERDATA_MAT3 => {
            if compare_member_name(member, member_length, c"Row1".as_ptr()) {
                return LBC_TYPE_VECTOR;
            }
            if compare_member_name(member, member_length, c"Row2".as_ptr()) {
                return LBC_TYPE_VECTOR;
            }
            if compare_member_name(member, member_length, c"Row3".as_ptr()) {
                return LBC_TYPE_VECTOR;
            }
        }
        K_USERDATA_VERTEX => {
            if compare_member_name(member, member_length, c"pos".as_ptr()) {
                return LBC_TYPE_VECTOR;
            }
            if compare_member_name(member, member_length, c"normal".as_ptr()) {
                return LBC_TYPE_VECTOR;
            }
            if compare_member_name(member, member_length, c"uv".as_ptr()) {
                return userdata_index_to_type(K_USERDATA_VEC2);
            }
        }
        _ => {}
    }

    LBC_TYPE_ANY
}
