use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::functions::get_tag::get_tag;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_extern_type::TypeFunctionExternType;
use crate::type_aliases::lua_state::lua_State;
use core::ffi::c_int;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_pushnil::lua_pushnil;

#[allow(non_snake_case)]
pub unsafe fn get_read_parent(l: *mut lua_State) -> c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.parent: expected 1 arguments, but got {}",
                argument_count
            ),
        );
    }

    let self_ty = get_type_user_data(l, 1);
    let tfct = get_type_function_type_id::<TypeFunctionExternType>(self_ty);

    if tfct.is_null() {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.parent: expected self to be a class, but got {} instead",
                get_tag(l, self_ty)
            ),
        );
    }

    if let Some(read_parent) = (*tfct).read_parent {
        alloc_type_user_data(l, (*read_parent).type_variant.clone(), false);
    } else {
        lua_pushnil(vm_l);
    }

    1
}
