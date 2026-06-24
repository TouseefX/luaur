use luaur_vm::functions::lua_l_checkinteger::lua_l_checkinteger;
use luaur_vm::functions::lua_pushinteger::lua_pushinteger;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub unsafe fn c_yielding_iterator_continuation(
    L: *mut lua_State,
    _status: core::ffi::c_int,
) -> core::ffi::c_int {
    let index = lua_l_checkinteger(L, 2);
    lua_pushinteger(L, index + 1);
    lua_pushinteger(L, index + 1);
    2
}
