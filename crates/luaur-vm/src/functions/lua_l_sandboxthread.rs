use crate::functions::lua_pushvalue::lua_pushvalue;
use crate::functions::lua_replace::lua_replace;
use crate::functions::lua_setfield::lua_setfield;
use crate::functions::lua_setmetatable::lua_setmetatable;
use crate::functions::lua_setreadonly::lua_setreadonly;
use crate::functions::lua_setsafeenv::lua_setsafeenv;
use crate::macros::lua_globalsindex::LUA_GLOBALSINDEX;
use crate::macros::lua_newtable::lua_newtable;
use crate::type_aliases::lua_state::lua_State;
use core::ffi::{c_char, c_int};

#[allow(non_snake_case)]
pub unsafe fn lua_l_sandboxthread(L: *mut lua_State) {
    // create new global table that proxies reads to original table
    lua_newtable(L);

    lua_newtable(L);

    // The dependency card for lua_pushvalue shows a 0-arg stub.
    // We must transmute it to the real signature to pass arguments.
    type LuaPushValueFn = unsafe extern "C" fn(*mut lua_State, c_int);
    let lua_pushvalue_ptr: LuaPushValueFn = core::mem::transmute(lua_pushvalue as *const ());
    lua_pushvalue_ptr(L, LUA_GLOBALSINDEX);

    type LuaSetFieldFn = unsafe extern "C" fn(*mut lua_State, c_int, *const c_char);
    let lua_setfield_ptr: LuaSetFieldFn = core::mem::transmute(lua_setfield as *const ());
    lua_setfield_ptr(L, -2, c"__index".as_ptr());

    type LuaSetReadOnlyFn = unsafe extern "C" fn(*mut lua_State, c_int, c_int);
    let lua_setreadonly_ptr: LuaSetReadOnlyFn = core::mem::transmute(lua_setreadonly as *const ());
    lua_setreadonly_ptr(L, -1, 1);

    type LuaSetMetatableFn = unsafe extern "C" fn(*mut lua_State, c_int) -> c_int;
    let lua_setmetatable_ptr: LuaSetMetatableFn =
        core::mem::transmute(lua_setmetatable as *const ());
    lua_setmetatable_ptr(L, -2);

    // we can set safeenv now although it's important to set it to false if code is loaded twice into the thread
    type LuaReplaceFn = unsafe extern "C" fn(*mut lua_State, c_int);
    let lua_replace_ptr: LuaReplaceFn = core::mem::transmute(lua_replace as *const ());
    lua_replace_ptr(L, LUA_GLOBALSINDEX);

    type LuaSetSafeEnvFn = unsafe extern "C" fn(*mut lua_State, c_int, c_int);
    let lua_setsafeenv_ptr: LuaSetSafeEnvFn = core::mem::transmute(lua_setsafeenv as *const ());
    lua_setsafeenv_ptr(L, LUA_GLOBALSINDEX, 1);
}
