use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn passthrough_call_varadic_continuation(
    l: *mut lua_State,
    _status: core::ffi::c_int,
) -> core::ffi::c_int {
    lua_gettop(l)
}
