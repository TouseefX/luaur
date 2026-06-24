use core::ffi::c_int;

use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::type_aliases::lua_state::lua_State;

pub unsafe fn direct_field_access_push_minus_one(L: *mut lua_State) -> c_int {
    lua_pushnumber(L, -1.0);
    1
}
