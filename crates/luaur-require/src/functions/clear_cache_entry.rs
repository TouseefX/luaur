use core::ffi::c_char;

use luaur_vm::functions::lua_l_findtable::luaL_findtable;
use luaur_vm::functions::lua_pushnil::lua_pushnil;
use luaur_vm::functions::lua_setfield::lua_setfield;
use luaur_vm::functions::lua_tolstring::lua_tolstring;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX;
use luaur_vm::records::lua_state::lua_State;

// C++ RequireImpl.cpp: `static const char* requiredCacheTableKey = "_MODULES";`
const required_cache_table_key: *const c_char = c"_MODULES".as_ptr();

pub fn clear_cache_entry(l: *mut lua_State) -> i32 {
    unsafe {
        let cache_key = lua_tolstring(l, 1, core::ptr::null_mut());

        luaL_findtable(l, LUA_REGISTRYINDEX, required_cache_table_key, 1);

        lua_pushnil(l);

        lua_setfield(l, -2, cache_key);

        lua_pop(l, 1);
    }
    0
}
