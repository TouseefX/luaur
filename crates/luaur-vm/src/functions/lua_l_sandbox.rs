use crate::functions::lua_getmetatable::lua_getmetatable;
use crate::functions::lua_next::lua_next;
use crate::functions::lua_pushnil::lua_pushnil;
use crate::functions::lua_setreadonly::lua_setreadonly;
use crate::functions::lua_setsafeenv::lua_setsafeenv;
use crate::functions::lua_type::lua_type;
use crate::macros::lua_globalsindex::LUA_GLOBALSINDEX;
use crate::macros::lua_istable::lua_istable;
use crate::macros::lua_pop::lua_pop;
use crate::macros::lua_pushliteral::lua_pushliteral;
use crate::type_aliases::lua_state::lua_State;

#[allow(non_snake_case)]
pub unsafe fn lua_l_sandbox(l: *mut lua_State) {
    // The dependency stubs for lua_type, lua_setreadonly, etc. are currently 0-arg stubs.
    // We cast them to their real signatures as defined in the C++ source and required by the VM.
    type LuaTypeFn = unsafe extern "C" fn(*mut lua_State, core::ffi::c_int) -> core::ffi::c_int;
    let lua_type_ptr: LuaTypeFn = core::mem::transmute(lua_type as *const ());

    type LuaNextFn = unsafe extern "C" fn(*mut lua_State, core::ffi::c_int) -> core::ffi::c_int;
    let lua_next_ptr: LuaNextFn = core::mem::transmute(lua_next as *const ());

    type LuaGetMetatableFn =
        unsafe extern "C" fn(*mut lua_State, core::ffi::c_int) -> core::ffi::c_int;
    let lua_getmetatable_ptr: LuaGetMetatableFn =
        core::mem::transmute(lua_getmetatable as *const ());

    type LuaSetReadOnlyFn =
        unsafe extern "C" fn(*mut lua_State, core::ffi::c_int, core::ffi::c_int);
    let lua_setreadonly_ptr: LuaSetReadOnlyFn = core::mem::transmute(lua_setreadonly as *const ());

    type LuaSetSafeEnvFn = unsafe extern "C" fn(*mut lua_State, core::ffi::c_int, core::ffi::c_int);
    let lua_setsafeenv_ptr: LuaSetSafeEnvFn = core::mem::transmute(lua_setsafeenv as *const ());

    // set all libraries to read-only
    lua_pushnil(l);
    while lua_next_ptr(l, LUA_GLOBALSINDEX) != 0 {
        // lua_istable! macro uses lua_type internally; we check the type using our transmuted pointer.
        if lua_type_ptr(l, -1) == crate::enums::lua_type::lua_Type::LUA_TTABLE as i32 {
            lua_setreadonly_ptr(l, -1, 1);
        }
        lua_pop(l, 1);
    }

    // set all builtin metatables to read-only
    lua_pushliteral(l as *mut core::ffi::c_void, c"".as_ptr());
    if lua_getmetatable_ptr(l, -1) != 0 {
        lua_setreadonly_ptr(l, -1, 1);
        lua_pop(l, 2);
    } else {
        lua_pop(l, 1);
    }

    // set globals to readonly and activate safeenv since the env is immutable
    lua_setreadonly_ptr(l, LUA_GLOBALSINDEX, 1);
    lua_setsafeenv_ptr(l, LUA_GLOBALSINDEX, 1);
}
