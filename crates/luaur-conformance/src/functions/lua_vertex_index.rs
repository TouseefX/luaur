use core::ffi::{c_int, CStr};

use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::functions::lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::lua_vec_2_push::lua_vec_2_push;
use crate::functions::lua_vertex_get::lua_vertex_get;
use crate::records::vertex::Vertex;

pub unsafe extern "C" fn lua_vertex_index(L: *mut lua_State) -> c_int {
    let v = lua_vertex_get(L, 1);
    let name_ptr = luaL_checkstring!(L, 2);
    let name = CStr::from_ptr(name_ptr).to_str().unwrap_or("");

    if name == "pos" {
        lua_pushvector_lua_state_f32_f32_f32(L, (*v).pos[0], (*v).pos[1], (*v).pos[2]);
        return 1;
    }

    if name == "normal" {
        lua_pushvector_lua_state_f32_f32_f32(L, (*v).normal[0], (*v).normal[1], (*v).normal[2]);
        return 1;
    }

    if name == "uv" {
        let uv = lua_vec_2_push(L);
        (*uv).x = (*v).uv[0];
        (*uv).y = (*v).uv[1];
        return 1;
    }

    if name == "sizeof" {
        lua_pushnumber(L, core::mem::size_of::<Vertex>() as f64);
        return 1;
    }

    lua_l_error_l(
        L,
        c"%s is not a valid member of vertex".as_ptr(),
        format_args!("{name} is not a valid member of vertex"),
    );
    0
}
