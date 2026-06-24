use core::ffi::c_char;
use luaur_vm::functions::lua_getfield::lua_getfield;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
use luaur_vm::functions::lua_setfield::lua_setfield;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::macros::lua_l_error::luaL_error;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX;
use luaur_vm::records::lua_state::lua_State;

const REQUIRED_CACHE_TABLE_KEY: *const c_char = c"_MODULES".as_ptr();
pub const K_REQUIRE_STACK_VALUES: i32 = 4;

pub fn lua_requirecont(l: *mut lua_State, _status: i32) -> i32 {
    unsafe {
        luaur_common::LUAU_ASSERT!(lua_gettop(l) >= K_REQUIRE_STACK_VALUES);
        let num_results = lua_gettop(l) - K_REQUIRE_STACK_VALUES;
        let cache_key = luaL_checkstring!(l, 2);

        if num_results > 1 {
            luaL_error!(l, "module must return a single value");
            return 0;
        }

        if num_results == 1 {
            lua_getfield(l, LUA_REGISTRYINDEX, REQUIRED_CACHE_TABLE_KEY);
            lua_pushvalue(l, -2);
            lua_setfield(l, -2, cache_key);
            lua_pop(l, 1);
        }

        num_results
    }
}
