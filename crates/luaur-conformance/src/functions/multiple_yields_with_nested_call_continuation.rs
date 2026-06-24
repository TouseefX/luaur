use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_checkinteger::lua_l_checkinteger;
use luaur_vm::functions::lua_l_checkstack::lua_l_checkstack;
use luaur_vm::functions::lua_pushinteger::lua_pushinteger;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::functions::lua_replace::lua_replace;
use luaur_vm::functions::lua_yield::lua_yield;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub unsafe fn multiple_yields_with_nested_call_continuation(
    L: *mut lua_State,
    _status: core::ffi::c_int,
) -> core::ffi::c_int {
    let state = lua_l_checkinteger(L, 3);

    lua_l_checkstack(L, 1, "cnestedmultiyieldcont");
    lua_pushinteger(L, state + 1);
    lua_replace(L, 3);

    if state == 0 {
        lua_yield(L, lua_gettop(L) - 3)
    } else if state == 1 {
        lua_pushnumber(L, lua_l_checkinteger(L, 1) as f64 + 200.0);
        lua_yield(L, 1)
    } else {
        lua_pushnumber(L, lua_l_checkinteger(L, 1) as f64 + 210.0);
        1
    }
}
