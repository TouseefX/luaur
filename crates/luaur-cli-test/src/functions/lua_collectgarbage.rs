//! Faithful port of `lua_collectgarbage` from `CLI/src/Repl.cpp:118`.
//! Mirrors `luau-repl-cli/src/functions/lua_collectgarbage.rs`, adapted to the
//! cli-test module layout (`lua_State` lives under `records::lua_state`).

use core::ffi::CStr;

use luaur_vm::enums::lua_gc_op::lua_GCOp;
use luaur_vm::functions::lua_gc::lua_gc;
use luaur_vm::functions::lua_l_optlstring::lua_l_optlstring;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::macros::lua_l_error::luaL_error;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn lua_collectgarbage(l: *mut lua_State) -> core::ffi::c_int {
    let option = lua_l_optlstring(l, 1, c"collect".as_ptr(), core::ptr::null_mut());
    let option = CStr::from_ptr(option);

    if option.to_bytes() == b"collect" {
        lua_gc(l, lua_GCOp::LUA_GCCOLLECT as core::ffi::c_int, 0);
        return 0;
    }

    if option.to_bytes() == b"count" {
        let c = lua_gc(l, lua_GCOp::LUA_GCCOUNT as core::ffi::c_int, 0);
        lua_pushnumber(l, c as f64);
        return 1;
    }

    luaL_error!(l, "collectgarbage must be called with 'count' or 'collect'");
    0
}
