use core::ffi::{c_char, c_int, CStr};

use luaur_vm::functions::lua_getfield::lua_getfield;
use luaur_vm::functions::lua_l_findtable::luaL_findtable;
use luaur_vm::functions::lua_remove::lua_remove;
use luaur_vm::functions::lua_type::lua_type;
use luaur_vm::macros::lua_isnil::lua_isnil;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX;
use luaur_vm::records::lua_state::lua_State;

use alloc::string::String;

const registered_cache_table_key: *const c_char = c"registeredCacheTableKey".as_ptr();

pub fn check_registered_modules(l: *mut lua_State, path: *const c_char) -> c_int {
    unsafe {
        luaL_findtable(l, LUA_REGISTRYINDEX, registered_cache_table_key, 1);

        let path_str = CStr::from_ptr(path).to_string_lossy();
        let mut path_lower = String::from(path_str);

        for c in path_lower.as_mut_vec() {
            if *c >= b'A' && *c <= b'Z' {
                *c += b'a' - b'A';
            }
        }

        let path_lower_c = String::from(path_lower.clone() + "\0");

        lua_getfield(l, -1, path_lower_c.as_ptr() as *const c_char);

        if lua_type(l, -1) == (luaur_vm::enums::lua_type::lua_Type::LUA_TNIL as i32) {
            lua_pop(l, 2);
            return 0;
        }

        lua_remove(l, -2);

        1
    }
}
