use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::records::type_function_unknown_type::TypeFunctionUnknownType;
use crate::type_aliases::lua_state::lua_State;

#[allow(non_snake_case)]
pub unsafe fn create_unknown(l: *mut lua_State) -> core::ffi::c_int {
    alloc_type_user_data(
        l,
        crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant::Unknown(
            TypeFunctionUnknownType {
                _unused: core::option::Option::None,
            },
        ),
        false,
    );

    1
}
