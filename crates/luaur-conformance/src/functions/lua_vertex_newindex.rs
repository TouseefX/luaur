use core::ffi::{c_char, CStr};
use luaur_vm::functions::lua_l_checkvector::lua_l_checkvector;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::lua_vec_2_get::lua_vec_2_get;
use crate::functions::lua_vertex_get::lua_vertex_get;
use crate::records::vertex::Vertex;

#[allow(non_snake_case)]
pub unsafe fn lua_vertex_newindex(L: *mut lua_State) -> i32 {
    let v = lua_vertex_get(L, 1);
    let name_ptr = luaL_checkstring!(L, 2);
    let name = CStr::from_ptr(name_ptr as *const c_char)
        .to_str()
        .unwrap_or("");

    if name == "pos" {
        let pos = lua_l_checkvector(L, 3);
        (*v).pos[0] = *pos;
        (*v).pos[1] = *pos.add(1);
        (*v).pos[2] = *pos.add(2);
    } else if name == "normal" {
        let normal = lua_l_checkvector(L, 3);
        (*v).normal[0] = *normal;
        (*v).normal[1] = *normal.add(1);
        (*v).normal[2] = *normal.add(2);
    } else if name == "uv" {
        let uv = lua_vec_2_get(L, 3);
        (*v).uv[0] = (*uv).x;
        (*v).uv[1] = (*uv).y;
    } else {
        let msg = b"%s is not a writable member of vertex\0";
        lua_l_error_l(
            L,
            msg.as_ptr() as *const c_char,
            core::format_args!("{} is not a writable member of vertex", name),
        );
    }

    0
}
