use crate::functions::get_tag::get_tag;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::type_aliases::lua_state::lua_State;
use core::ffi::CStr;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_pushboolean::lua_pushboolean;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;

pub unsafe fn check_tag(l: *mut lua_State) -> core::ffi::c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 2 {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!("type.is: expected 2 arguments, but got {}", argument_count),
        );
    }

    let self_ = get_type_user_data(l, 1);
    let arg = luaL_checkstring!(vm_l, 2);
    let arg_str = CStr::from_ptr(arg).to_string_lossy();

    let tag = get_tag(l, self_);
    lua_pushboolean(vm_l, if tag == arg_str { 1 } else { 0 });
    1
}
