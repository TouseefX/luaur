use luaur_vm::functions::lua_l_checkinteger::lua_l_checkinteger;
use luaur_vm::functions::lua_l_checkstack::lua_l_checkstack;
use luaur_vm::functions::lua_pushinteger::lua_pushinteger;
use luaur_vm::functions::lua_replace::lua_replace;
use luaur_vm::functions::lua_yield::lua_yield;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub unsafe fn multipleYieldsContinuation(
    L: *mut lua_State,
    status: core::ffi::c_int,
) -> core::ffi::c_int {
    let base = lua_l_checkinteger(L, 1);
    let pos = lua_l_checkinteger(L, 2) + 1;

    lua_l_checkstack(L, 1, "cmultiyieldcont");
    lua_pushinteger(L, pos);
    lua_replace(L, 2);

    lua_l_checkstack(L, 1, "cmultiyieldcont");

    if pos < 4 {
        lua_pushinteger(L, base + pos);
        lua_yield(L, 1)
    } else {
        lua_pushinteger(L, base + pos);
        1
    }
}
