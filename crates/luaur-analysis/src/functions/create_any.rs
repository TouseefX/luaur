use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::records::type_function_any_type::TypeFunctionAnyType;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;

#[allow(non_snake_case)]
pub unsafe fn create_any(l: *mut lua_State) -> core::ffi::c_int {
    alloc_type_user_data(
        l,
        TypeFunctionTypeVariant::Any(TypeFunctionAnyType::default()),
        false,
    );
    1
}
