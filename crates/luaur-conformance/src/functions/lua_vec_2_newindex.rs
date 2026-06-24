use core::ffi::{c_char, CStr};
use luaur_vm::functions::lua_l_checknumber::luaL_checknumber;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::lua_vec_2_get::lua_vec_2_get;
use crate::records::vec_2_conformance_ir_hooks::Vec2;

#[allow(non_snake_case)]
pub unsafe fn lua_vec_2_newindex(L: *mut lua_State) -> i32 {
    let v = lua_vec_2_get(L, 1);
    let name_ptr = luaL_checkstring!(L, 2);
    let name = CStr::from_ptr(name_ptr as *const c_char)
        .to_str()
        .unwrap_or("");
    let value = luaL_checknumber(L, 3) as f32;

    if name == "X" {
        (*v).x = value;
    } else if name == "Y" {
        (*v).y = value;
    } else {
        let msg = b"%s is not a writable member of vec2\0";
        lua_l_error_l(
            L,
            msg.as_ptr() as *const c_char,
            core::format_args!("{} is not a writable member of vec2", name),
        );
    }

    0
}
