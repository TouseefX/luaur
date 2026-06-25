use core::ffi::c_char;

use luaur_vm::functions::lua_setfield::lua_setfield;
use luaur_vm::macros::lua_newtable::lua_newtable;
use luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX;

// C++ RequireImpl.cpp: `static const char* requiredCacheTableKey = "_MODULES";`
const required_cache_table_key: *const c_char = c"_MODULES".as_ptr();

pub fn clear_cache(l: *mut luaur_vm::records::lua_state::lua_State) -> i32 {
    unsafe {
        lua_newtable(l);

        lua_setfield(l, LUA_REGISTRYINDEX, required_cache_table_key);
    }
    0
}
