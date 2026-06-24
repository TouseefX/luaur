use core::ffi::c_char;

use luaur_vm::functions::lua_setfield::lua_setfield;
use luaur_vm::macros::lua_newtable::lua_newtable;
use luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX;

// C++ RequireImpl.cpp: `static const char* requiredCacheTableKey = "_MODULES";`
const required_cache_table_key: *const c_char = c"_MODULES".as_ptr();

pub fn clear_cache(l: *mut luaur_vm::records::lua_state::lua_State) -> i32 {
    unsafe {
        lua_newtable(l);

        // The dependency card for lua_setfield shows a stub `fn lua_setfield()`,
        // but the C++ source and the VM's lapi.cpp (referenced in the card)
        // define it as `void lua_setfield(lua_State *L, int idx, const char *k)`.
        // We must cast the function pointer to the real signature to call it.
        let lua_setfield_ptr: unsafe extern "C" fn(
            *mut luaur_vm::records::lua_state::lua_State,
            i32,
            *const c_char,
        ) = core::mem::transmute(lua_setfield as *const ());

        lua_setfield_ptr(l, LUA_REGISTRYINDEX, required_cache_table_key);
    }
    0
}
