use core::ffi::c_int;

use crate::functions::get_int_64::get_int_64;
use crate::functions::push_int_64::push_int_64;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn int_64_idiv(l: *mut lua_State) -> c_int {
    let value = ((get_int_64(l, 1) as f64) / (get_int_64(l, 2) as f64)).floor() as i64;
    push_int_64(l, value);
    1
}
