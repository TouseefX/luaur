use luaur_vm::functions::lua_touserdatatagged::lua_touserdatatagged;
use luaur_vm::macros::lua_l_typeerror::luaL_typeerror;
use luaur_vm::records::lua_state::lua_State;

use crate::records::vec_2_conformance_ir_hooks::Vec2;

#[allow(non_snake_case)]
pub unsafe fn lua_vec_2_get(L: *mut lua_State, idx: i32) -> *mut Vec2 {
    let a = lua_touserdatatagged(L, idx as i32, 12) as *mut Vec2;

    if !a.is_null() {
        return a;
    }

    luaL_typeerror!(L, idx as i32, "vec2");
    core::ptr::null_mut()
}
