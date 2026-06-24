use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::macros::lua_isnil::lua_isnil;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_tonumber::lua_tonumber;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub unsafe fn passthrough_call_more_results_continuation(
    l: *mut lua_State,
    _status: core::ffi::c_int,
) -> core::ffi::c_int {
    LUAU_ASSERT!(lua_gettop(l) == 13);

    for _ in 0..9 {
        LUAU_ASSERT!(lua_isnil!(l, -1));
        lua_pop(l, 1);
    }

    LUAU_ASSERT!(lua_tonumber!(l, -1) == 0.5);
    1
}
