use std::ffi::CStr;

use luaur_vm::functions::lua_checkstack::lua_checkstack;
use luaur_vm::functions::lua_getinfo::lua_getinfo;
use luaur_vm::functions::lua_getupvalue::lua_getupvalue;
use luaur_vm::macros::lua_minstack::LUA_MINSTACK;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_tointeger::lua_tointeger;
use luaur_vm::records::lua_debug::LuaDebug;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_n_debug_get_up_value_yield(l: *mut lua_State) -> bool {
    lua_checkstack(l, LUA_MINSTACK);

    let mut ar: LuaDebug = core::mem::zeroed();
    assert_ne!(0, lua_getinfo(l, 1, c"f".as_ptr(), &mut ar));

    let upvalue = lua_getupvalue(l, -1, 1);
    assert!(!upvalue.is_null());
    assert_eq!(CStr::from_ptr(upvalue).to_bytes(), b"");
    assert_eq!(lua_tointeger!(l, -1), 5);
    lua_pop(l, 2);

    false
}
