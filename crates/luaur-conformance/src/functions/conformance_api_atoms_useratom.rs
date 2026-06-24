use core::ffi::{c_char, c_int};
use std::ffi::CStr;

use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_api_atoms_useratom(
    _l: *mut lua_State,
    s: *const c_char,
    _len: usize,
) -> i16 {
    if CStr::from_ptr(s) == c"string" {
        return 0;
    }

    if CStr::from_ptr(s) == c"important" {
        return 1;
    }

    -1
}
