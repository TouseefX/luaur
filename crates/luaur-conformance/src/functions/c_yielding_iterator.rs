use luaur_vm::functions::lua_l_checkinteger::lua_l_checkinteger;
use luaur_vm::functions::lua_pushinteger::lua_pushinteger;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub unsafe fn c_yielding_iterator(L: *mut lua_State) -> i32 {
    let max = lua_l_checkinteger(L, 1);
    let index = lua_l_checkinteger(L, 2);

    if index >= max {
        return 0; // nil: end iteration
    }

    lua_pushinteger(L, index + 1);
    luaur_vm::functions::lua_yield::lua_yield(L, 1)
}
