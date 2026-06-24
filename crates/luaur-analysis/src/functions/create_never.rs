use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::records::type_function_never_type::TypeFunctionNeverType;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;

#[allow(non_snake_case)]
pub unsafe fn create_never(l: *mut lua_State) -> core::ffi::c_int {
    alloc_type_user_data(
        l,
        TypeFunctionTypeVariant::Never(TypeFunctionNeverType::default()),
        false,
    );
    1
}
