use core::ffi::c_int;

use crate::functions::get_int_64::get_int_64;
use luaur_vm::functions::lua_pushboolean::lua_pushboolean;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn int_64_eq(l: *mut lua_State) -> c_int {
    lua_pushboolean(l, (get_int_64(l, 1) == get_int_64(l, 2)) as c_int);
    1
}
