use core::ffi::c_int;

use crate::functions::push_int_64::push_int_64;
use luaur_vm::functions::lua_l_checknumber::lua_l_checknumber;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn int_64_ctor(l: *mut lua_State) -> c_int {
    let value = lua_l_checknumber(l, 1);
    push_int_64(l, value as i64);
    1
}
