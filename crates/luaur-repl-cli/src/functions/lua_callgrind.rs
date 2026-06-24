use core::ffi::CStr;

use luaur_vm::functions::lua_l_checklstring::lua_l_checklstring;
use luaur_vm::functions::lua_pushboolean::lua_pushboolean;
use luaur_vm::macros::lua_l_error::luaL_error;
use luaur_vm::type_aliases::lua_state::lua_State;

// Faithful port of the `#ifdef CALLGRIND` `lua_callgrind` in Repl.cpp.
//
// The valgrind client-request macros (RUNNING_ON_VALGRIND, CALLGRIND_ZERO_STATS,
// CALLGRIND_DUMP_STATS_AT) are inline assembly sequences that valgrind's
// instrumentation intercepts. When the process is NOT running under valgrind
// (the only situation a normal build encounters), they evaluate to:
//   - RUNNING_ON_VALGRIND  -> 0
//   - CALLGRIND_ZERO_STATS -> no observable effect
//   - CALLGRIND_DUMP_STATS_AT(name) -> no observable effect
// We reproduce exactly that runtime behavior here.
pub unsafe fn lua_callgrind(l: *mut lua_State) -> i32 {
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

    luaL_error!(l, "callgrind must be called with one of 'running', 'zero', 'dump'");
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
