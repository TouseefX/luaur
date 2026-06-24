use crate::functions::k_int_64_tag::K_INT_64_TAG;
use luaur_vm::functions::lua_isnumber::lua_isnumber;
use luaur_vm::functions::lua_touserdatatagged::lua_touserdatatagged;
use luaur_vm::macros::lua_l_typeerror::luaL_typeerror;
use luaur_vm::macros::lua_tointeger::lua_tointeger;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub fn get_int_64(l: *mut lua_State, idx: i32) -> i64 {
    unsafe {
        let p = lua_touserdatatagged(l, idx, K_INT_64_TAG);
        if !p.is_null() {
            return *(p as *const i64);
        }

        if lua_isnumber(l, idx) != 0 {
            return lua_tointeger!(l, idx) as i64;
        }

        luaL_typeerror!(l, 1, "int64");
    }
}
