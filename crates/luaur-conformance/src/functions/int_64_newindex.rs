use core::ffi::{c_int, CStr};

use crate::functions::k_int_64_tag::K_INT_64_TAG;
use luaur_vm::functions::lua_l_checknumber::lua_l_checknumber;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_touserdatatagged::lua_touserdatatagged;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::macros::lua_l_typeerror::luaL_typeerror;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn int_64_newindex(l: *mut lua_State) -> c_int {
    let p = lua_touserdatatagged(l, 1, K_INT_64_TAG);
    if p.is_null() {
        luaL_typeerror!(l, 1, "int64");
    }

    let name_ptr = luaL_checkstring!(l, 2);
    let name = CStr::from_ptr(name_ptr).to_bytes();

    if name == b"value" {
        let value = lua_l_checknumber(l, 3);
        *(p as *mut i64) = value as i64;
        return 0;
    }

    let name = CStr::from_ptr(name_ptr).to_string_lossy();
    lua_l_error_l(
        l,
        c"unknown field %s".as_ptr(),
        format_args!("unknown field {name}"),
    );
    0
}
