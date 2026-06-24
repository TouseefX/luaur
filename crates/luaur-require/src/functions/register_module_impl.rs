use alloc::ffi::CString;
use core::ffi::c_char;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_insert::lua_insert;
use luaur_vm::functions::lua_l_argerror_l::luaL_argerrorL;
use luaur_vm::functions::lua_l_checklstring::lua_l_checklstring;
use luaur_vm::functions::lua_l_findtable::luaL_findtable;
use luaur_vm::functions::lua_pushstring::lua_pushstring;
use luaur_vm::functions::lua_replace::lua_replace;
use luaur_vm::functions::lua_settable::lua_settable;
use luaur_vm::macros::lua_l_error::luaL_error;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX;
use luaur_vm::records::lua_state::lua_State;

const REGISTERED_CACHE_TABLE_KEY: *const c_char = c"_REGISTEREDMODULES".as_ptr();

pub fn register_module_impl(l: *mut lua_State) -> i32 {
    unsafe {
        if lua_gettop(l) != 2 {
            luaL_error!(
                l,
                "expected 2 arguments: aliased require path and desired result"
            );
            return 0;
        }

        let mut len = 0usize;
        let path = lua_l_checklstring(l, 1, &mut len);
        if len == 0 || *path != b'@' as c_char {
            luaL_argerrorL(l, 1, "path must begin with '@'");
        }

        let mut path_lower = core::slice::from_raw_parts(path as *const u8, len).to_vec();
        for byte in &mut path_lower {
            if *byte >= b'A' && *byte <= b'Z' {
                *byte += b'a' - b'A';
            }
        }

        let path_lower = CString::new(path_lower).unwrap();
        lua_pushstring(l, path_lower.as_ptr());
        lua_replace(l, 1);

        luaL_findtable(l, LUA_REGISTRYINDEX, REGISTERED_CACHE_TABLE_KEY, 1);
        lua_insert(l, 1);
        lua_settable(l, 1);
        lua_pop(l, 1);

        0
    }
}
