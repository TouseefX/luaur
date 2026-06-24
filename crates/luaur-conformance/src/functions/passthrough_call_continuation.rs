use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::macros::lua_tonumber::lua_tonumber;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub unsafe fn passthrough_call_continuation(
    L: *mut lua_State,
    _status: core::ffi::c_int,
) -> core::ffi::c_int {
    LUAU_ASSERT!(lua_gettop(L) == 4);
    LUAU_ASSERT!(lua_tonumber!(L, -1) == 0.5);
    1
}
