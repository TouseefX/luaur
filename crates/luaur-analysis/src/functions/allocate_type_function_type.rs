use crate::functions::get_type_function_runtime::get_type_function_runtime;
use crate::records::type_function_type::TypeFunctionType;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;

pub unsafe fn allocate_type_function_type(
    l: *mut lua_State,
    type_variant: TypeFunctionTypeVariant,
) -> *mut TypeFunctionType {
    let ctx = get_type_function_runtime(l);
    (*ctx)
        .type_arena
        .allocate(TypeFunctionType::new(type_variant))
}
