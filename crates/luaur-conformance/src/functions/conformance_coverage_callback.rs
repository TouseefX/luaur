use core::ffi::{c_char, c_int, c_void};

use luaur_vm::functions::lua_objlen::lua_objlen;
use luaur_vm::functions::lua_pushinteger::lua_pushinteger;
use luaur_vm::functions::lua_pushstring::lua_pushstring;
use luaur_vm::functions::lua_rawseti::lua_rawseti;
use luaur_vm::functions::lua_setfield::lua_setfield;
use luaur_vm::macros::lua_newtable::lua_newtable;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_coverage_callback(
    context: *mut c_void,
    function: *const c_char,
    linedefined: c_int,
    depth: c_int,
    hits: *const c_int,
    size: usize,
) {
    let l = context as *mut lua_State;

    lua_newtable(l);

    lua_pushstring(l, function);
    lua_setfield(l, -2, c"name".as_ptr());

    lua_pushinteger(l, linedefined);
    lua_setfield(l, -2, c"linedefined".as_ptr());

    lua_pushinteger(l, depth);
    lua_setfield(l, -2, c"depth".as_ptr());

    for i in 0..size {
        let hit = *hits.add(i);

        if hit != -1 {
            lua_pushinteger(l, hit);
            lua_rawseti(l, -2, i as c_int);
        }
    }

    lua_rawseti(l, -2, lua_objlen(l, -2) + 1);
}
