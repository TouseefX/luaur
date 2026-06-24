use crate::functions::lua_pushinteger::lua_pushinteger;
use crate::functions::lua_setfield::lua_setfield;
use crate::type_aliases::lua_state::lua_State;
use core::ffi::{c_char, c_int};

pub fn setfield(L: *mut lua_State, key: &str, value: i32) {
    let key_bytes = key.as_bytes();
    let mut buf = key_bytes.to_vec();
    buf.push(0);
    let key_c: *const c_char = buf.as_ptr() as *const c_char;

    unsafe {
        lua_pushinteger(L, value as c_int);

        // The dependency lua_setfield is currently a stub with no arguments in the crate,
        // but the C++ source and the logic of this function require passing arguments.
        // We cast the function pointer to the correct signature to satisfy the call.
        let set_field: unsafe extern "C" fn(*mut lua_State, c_int, *const c_char) =
            core::mem::transmute(lua_setfield as *const ());

        set_field(L, -2, key_c);
    }
}
