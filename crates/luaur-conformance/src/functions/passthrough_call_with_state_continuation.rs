use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_checkinteger::lua_l_checkinteger;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub unsafe fn passthrough_call_with_state_continuation(
    L: *mut lua_State,
    _status: core::ffi::c_int,
) -> core::ffi::c_int {
    LUAU_ASSERT!(lua_l_checkinteger(L, 1) == 42);

    lua_gettop(L) - 1
}
