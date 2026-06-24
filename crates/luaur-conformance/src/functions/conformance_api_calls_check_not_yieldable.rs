use core::ffi::c_int;

use luaur_vm::functions::lua_isyieldable::lua_isyieldable;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn conformance_api_calls_check_not_yieldable(l: *mut lua_State) -> c_int {
    assert_eq!(lua_isyieldable(l), 0);
    0
}
