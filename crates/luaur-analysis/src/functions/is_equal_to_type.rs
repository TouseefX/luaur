use crate::functions::get_type_user_data::get_type_user_data;
use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_pushboolean::lua_pushboolean;

pub unsafe fn is_equal_to_type(l: *mut lua_State) -> i32 {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 2 {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!("expected 2 arguments, but got {}", argument_count),
        );
    }

    let self_ty = get_type_user_data(l, 1);
    let arg = get_type_user_data(l, 2);

    lua_pushboolean(vm_l, (*self_ty).operator_eq(&*arg) as core::ffi::c_int);
    1
}
