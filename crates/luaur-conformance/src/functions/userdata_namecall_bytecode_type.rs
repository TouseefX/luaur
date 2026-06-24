use crate::functions::compare_member_name::compare_member_name;
use crate::functions::type_to_userdata_index::type_to_userdata_index;
use crate::functions::userdata_index_to_type::userdata_index_to_type;

#[allow(non_upper_case_globals)]
const LBC_TYPE_ANY: u8 = 0;
#[allow(non_upper_case_globals)]
const LBC_TYPE_NUMBER: u8 = 2;

const kUserdataColor: u8 = 1;
const kUserdataVec2: u8 = 2;
const kUserdataMat3: u8 = 3;

pub fn userdata_namecall_bytecode_type(
    type_: u8,
    member: *const core::ffi::c_char,
    member_length: usize,
) -> u8 {
    match type_to_userdata_index(type_) {
        kUserdataColor => {}
        kUserdataVec2 => {
            if compare_member_name(member, member_length, c"Dot".as_ptr()) {
                return LBC_TYPE_NUMBER;
            }

            if compare_member_name(member, member_length, c"Min".as_ptr()) {
                return userdata_index_to_type(kUserdataVec2);
            }
        }
        kUserdataMat3 => {}
        _ => {}
    }

    LBC_TYPE_ANY
}
