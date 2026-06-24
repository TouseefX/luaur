use crate::functions::get_type_function_runtime::get_type_function_runtime;
use crate::records::type_function_type_pack_var::TypeFunctionTypePackVar;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_pack_variant::TypeFunctionTypePackVariant;

pub unsafe fn allocate_type_function_type_pack(
    l: *mut lua_State,
    type_variant: TypeFunctionTypePackVariant,
) -> *mut TypeFunctionTypePackVar {
    let ctx = get_type_function_runtime(l);
    (*ctx)
        .type_pack_arena
        .allocate(TypeFunctionTypePackVar::new(type_variant))
}
