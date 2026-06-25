//! Faithful port of the `#ifdef CALLGRIND` `lua_callgrind` in
//! `CLI/src/Repl.cpp:139`. Mirrors
//! `luau-repl-cli/src/functions/lua_callgrind.rs`, adapted to the cli-test
//! module layout (`lua_State` lives under `records::lua_state`).
//!
//! The valgrind client-request macros (RUNNING_ON_VALGRIND, CALLGRIND_ZERO_STATS,
//! CALLGRIND_DUMP_STATS_AT) are inline assembly sequences that valgrind's
//! instrumentation intercepts. When the process is NOT running under valgrind
//! (the only situation a normal build encounters), they evaluate to:
//!   - RUNNING_ON_VALGRIND  -> 0
//!   - CALLGRIND_ZERO_STATS -> no observable effect
//!   - CALLGRIND_DUMP_STATS_AT(name) -> no observable effect
//! We reproduce exactly that runtime behavior here.

use core::ffi::CStr;

use luaur_vm::functions::lua_l_checklstring::lua_l_checklstring;
use luaur_vm::functions::lua_pushboolean::lua_pushboolean;
use luaur_vm::macros::lua_l_error::luaL_error;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn lua_callgrind(l: *mut lua_State) -> core::ffi::c_int {
    let mut len: usize = 0;
    let option = lua_l_checklstring(l, 1, &mut len as *mut usize);
    let option = CStr::from_ptr(option);

    if option.to_bytes() == b"running" {
        let r = running_on_valgrind();
        lua_pushboolean(l, r);
        return 1;
    }

    if option.to_bytes() == b"zero" {
        callgrind_zero_stats();
        return 0;
    }

    if option.to_bytes() == b"dump" {
        let mut name_len: usize = 0;
        let name = lua_l_checklstring(l, 2, &mut name_len as *mut usize);

        callgrind_dump_stats_at(name);
        return 0;
    }

    luaL_error!(
        l,
        "callgrind must be called with one of 'running', 'zero', 'dump'"
    );
    0
}

#[inline]
fn running_on_valgrind() -> core::ffi::c_int {
    // No valgrind instrumentation present: RUNNING_ON_VALGRIND evaluates to 0.
    0
}

#[inline]
fn callgrind_zero_stats() {
    // CALLGRIND_ZERO_STATS is a no-op outside valgrind.
}

#[inline]
unsafe fn callgrind_dump_stats_at(_name: *const core::ffi::c_char) {
    // CALLGRIND_DUMP_STATS_AT is a no-op outside valgrind.
}
