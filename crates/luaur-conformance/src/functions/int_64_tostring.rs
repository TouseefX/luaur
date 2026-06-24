use alloc::string::ToString;
use core::ffi::c_int;

use crate::functions::get_int_64::get_int_64;
use luaur_vm::functions::lua_pushlstring::lua_pushlstring;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn int_64_tostring(l: *mut lua_State) -> c_int {
    let string = get_int_64(l, 1).to_string();
    lua_pushlstring(l, string.as_ptr().cast(), string.len());
    1
}
