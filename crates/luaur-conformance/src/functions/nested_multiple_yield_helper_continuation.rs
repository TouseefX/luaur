use luaur_vm::functions::lua_l_checkinteger::lua_l_checkinteger;
use luaur_vm::functions::lua_pushinteger::lua_pushinteger;
use luaur_vm::macros::lua_upvalueindex::lua_upvalueindex;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub unsafe fn nested_multiple_yield_helper_continuation(
    l: *mut lua_State,
    _status: core::ffi::c_int,
) -> core::ffi::c_int {
    let context = lua_l_checkinteger(l, lua_upvalueindex(1));
    lua_pushinteger(l, 110 + context);
    1
}
