use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::functions::get_tag::get_tag;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_singleton_type::TypeFunctionSingletonType;
use crate::records::type_function_table_type::TypeFunctionTableType;
use crate::type_aliases::lua_state::lua_State;
use core::ffi::c_int;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_pushnil::lua_pushnil;

pub unsafe fn write_table_prop(l: *mut lua_State) -> c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 2 {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.writeproperty: expected 2 arguments, but got {}",
                argument_count
            ),
        );
    }

    let self_ty = get_type_user_data(l, 1);
    let tftt = get_type_function_type_id::<TypeFunctionTableType>(self_ty);
    if tftt.is_null() {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.writeproperty: expected self to be either a table, but got {} instead",
                get_tag(l, self_ty)
            ),
        );
    }

    let key = get_type_user_data(l, 2);
    let tfst = get_type_function_type_id::<TypeFunctionSingletonType>(key);
    if tfst.is_null() {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.writeproperty: expected to be given a singleton type, but got {} instead",
                get_tag(l, key)
            ),
        );
    }

    let tfsst = (*tfst).variant.get_if_1();
    if tfsst.is_none() {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.writeproperty: expected to be given a string singleton type, but got {} instead",
                get_tag(l, key)
            ),
        );
    }

    let key_name = &tfsst.unwrap().value;
    let prop = (*tftt).props.get(key_name);
    if prop.is_none() {
        lua_pushnil(vm_l);
        return 1;
    }

    if let Some(write_ty) = prop.unwrap().write_ty {
        alloc_type_user_data(l, (*write_ty).type_variant.clone(), false);
    } else {
        lua_pushnil(vm_l);
    }

    1
}
