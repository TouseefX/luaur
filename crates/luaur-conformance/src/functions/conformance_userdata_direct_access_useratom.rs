use core::ffi::{c_char, c_int};

use crate::functions::get_or_create_atom::get_or_create_atom;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_userdata_direct_access_useratom(
    _l: *mut lua_State,
    s: *const c_char,
    len: usize,
) -> i16 {
    let bytes = core::slice::from_raw_parts(s.cast::<u8>(), len);
    let name = core::str::from_utf8(bytes).unwrap_or("");
    get_or_create_atom(name) as c_int as i16
}
