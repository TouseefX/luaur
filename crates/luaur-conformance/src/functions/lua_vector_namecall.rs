use core::ffi::{c_char, CStr};
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::lua_vector_cross::lua_vector_cross;
use crate::functions::lua_vector_dot::lua_vector_dot;
use luaur_vm::functions::lua_namecallatom::lua_namecallatom;

#[allow(non_snake_case)]
pub unsafe fn lua_vector_namecall(L: *mut lua_State) -> i32 {
    let mut atom: i32 = 0;
    let str_ptr = lua_namecallatom(L, &mut atom);

    if !str_ptr.is_null() {
        let str_cstr = CStr::from_ptr(str_ptr);
        let str_slice = str_cstr.to_str().unwrap_or("");

        if str_slice == "Dot" {
            return lua_vector_dot(L);
        }

        if str_slice == "Cross" {
            return lua_vector_cross(L);
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
