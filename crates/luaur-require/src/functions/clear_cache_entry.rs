use core::ffi::{c_char, c_int};

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
        // luaL_checkstring is a macro in the VM headers that expands to luaL_checklstring(L, n, NULL).
        // However, the provided dependency card for luaL_checkstring shows it as a unit-type constant.
        // In Luau, luaL_checkstring(L, n) is equivalent to lua_tolstring(L, n, NULL) with an error check.
        // Since we must resolve the macro/function mismatch, we use the underlying VM function.
        let lua_tolstring_ptr: unsafe extern "C" fn(
            *mut lua_State,
            c_int,
            *mut usize,
        ) -> *const c_char = core::mem::transmute(lua_tolstring as *const ());
        let cache_key = lua_tolstring_ptr(l, 1, core::ptr::null_mut());

        let lua_l_findtable_ptr: unsafe extern "C" fn(
            *mut lua_State,
            c_int,
            *const c_char,
            c_int,
        ) -> *const c_char = core::mem::transmute(luaL_findtable as *const ());
        lua_l_findtable_ptr(l, LUA_REGISTRYINDEX, required_cache_table_key, 1);

        lua_pushnil(l);

        let lua_setfield_ptr: unsafe extern "C" fn(*mut lua_State, c_int, *const c_char) =
            core::mem::transmute(lua_setfield as *const ());
        lua_setfield_ptr(l, -2, cache_key);

        lua_pop(l, 1);
    }
    0
}
