use crate::functions::type_to_userdata_index::type_to_userdata_index;
use crate::functions::userdata_index_to_type::userdata_index_to_type;
use luaur_code_gen::enums::host_metamethod::HostMetamethod;

#[allow(non_upper_case_globals)]
const LBC_TYPE_ANY: u8 = 15;

const K_USERDATA_VEC2: u8 = 2;

pub fn userdata_metamethod_bytecode_type(lhs_ty: u8, rhs_ty: u8, method: HostMetamethod) -> u8 {
    match method {
        HostMetamethod::Add | HostMetamethod::Sub | HostMetamethod::Mul | HostMetamethod::Div => {
            if type_to_userdata_index(lhs_ty) == K_USERDATA_VEC2
                || type_to_userdata_index(rhs_ty) == K_USERDATA_VEC2
            {
                return userdata_index_to_type(K_USERDATA_VEC2);
            }
        }
        HostMetamethod::Minus => {
            if type_to_userdata_index(lhs_ty) == K_USERDATA_VEC2 {
                return userdata_index_to_type(K_USERDATA_VEC2);
            }
        }
        _ => {}
    }

    LBC_TYPE_ANY
}
