use core::ffi::c_int;

use crate::functions::get_int_64::get_int_64;
use crate::functions::push_int_64::push_int_64;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn int_64_add(l: *mut lua_State) -> c_int {
    push_int_64(l, get_int_64(l, 1) + get_int_64(l, 2));
    1
}
