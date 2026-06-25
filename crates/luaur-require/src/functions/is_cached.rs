use alloc::string::String;
use core::ffi::c_char;

use luaur_vm::functions::lua_getfield::lua_getfield;
use luaur_vm::functions::lua_l_findtable::luaL_findtable;
use luaur_vm::functions::lua_type::lua_type;
use luaur_vm::macros::lua_isnil::lua_isnil;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX;
use luaur_vm::records::lua_state::lua_State;

const required_cache_table_key: *const c_char = c"requiredCacheTableKey".as_ptr();

pub fn is_cached(l: *mut lua_State, key: &String) -> bool {
    unsafe {
        luaL_findtable(l, LUA_REGISTRYINDEX, required_cache_table_key, 1);

        let key_c = alloc::ffi::CString::new(key.as_str()).unwrap();
        lua_getfield(l, -1, key_c.as_ptr());

        let cached = lua_type(l, -1) != (luaur_vm::enums::lua_type::lua_Type::LUA_TNIL as i32);

        lua_pop(l, 2);

        cached
    }
}
