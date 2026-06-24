use core::ffi::{c_char, CStr};
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::lua_vec_2_clone::lua_vec_2_clone;
use crate::functions::lua_vec_2_dot::lua_vec_2_dot;
use crate::functions::lua_vec_2_get::lua_vec_2_get;
use crate::functions::lua_vec_2_min::lua_vec_2_min;
use crate::functions::lua_vec_2_reenter::lua_vec_2_reenter;
use luaur_vm::functions::lua_namecallatom::lua_namecallatom;

#[allow(non_snake_case)]
pub unsafe fn lua_vec_2_namecall(L: *mut lua_State) -> i32 {
    let str_ptr = lua_namecallatom(L, core::ptr::null_mut());

    if !str_ptr.is_null() {
        let str_cstr = CStr::from_ptr(str_ptr);
        let str_slice = str_cstr.to_str().unwrap_or("");

        if str_slice == "Dot" {
            let self_ptr = lua_vec_2_get(L, 1);
            return lua_vec_2_dot(L, self_ptr);
        }

        if str_slice == "Min" {
            let self_ptr = lua_vec_2_get(L, 1);
            return lua_vec_2_min(L, self_ptr);
        }

        if str_slice == "Clone" {
            let self_ptr = lua_vec_2_get(L, 1);
            return lua_vec_2_clone(L, self_ptr);
        }

        if str_slice == "Reenter" {
            let self_ptr = lua_vec_2_get(L, 1);
            return lua_vec_2_reenter(L, self_ptr);
        }
    }

    let arg1_ptr = luaL_checkstring!(L, 1);
    let arg1_cstr = CStr::from_ptr(arg1_ptr as *const c_char);
    let arg1_str = arg1_cstr.to_str().unwrap_or("");

    let msg = b"%s is not a valid method of vector\0";
    lua_l_error_l(
        L,
        msg.as_ptr() as *const c_char,
        core::format_args!("{} is not a valid method of vector", arg1_str),
    );

    0
}
