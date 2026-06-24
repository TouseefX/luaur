use luaur_vm::functions::lua_l_checkinteger::lua_l_checkinteger;
use luaur_vm::functions::lua_l_checkstack::lua_l_checkstack;
use luaur_vm::functions::lua_pushinteger::lua_pushinteger;
use luaur_vm::functions::lua_settop::lua_settop;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub unsafe fn multipleYields(L: *mut lua_State) -> i32 {
    lua_settop(L, 1);

    let base = lua_l_checkinteger(L, 1);

    lua_l_checkstack(L, 2, "cmultiyield");

    let pos: i32 = 1;

    lua_pushinteger(L, pos);

    lua_pushinteger(L, base + pos);

    luaur_vm::functions::lua_yield::lua_yield(L, 1)
}
