use core::ffi::c_int;
use core::sync::atomic::AtomicPtr;

use luaur_vm::functions::lua_callbacks::lua_callbacks;
use luaur_vm::functions::lua_rawcheckstack::lua_rawcheckstack;
use luaur_vm::macros::lua_l_error::luaL_error;
use luaur_vm::type_aliases::lua_state::lua_State;

// `replState` from Repl.cpp: the REPL's lua_State, used by the OS signal handler
// to arm the interrupt callback. Stored atomically so the async-signal handler
// (sigintHandler) can read it safely.
pub static REPL_STATE: AtomicPtr<lua_State> = AtomicPtr::new(core::ptr::null_mut());

// Ctrl-C handling. Matches the `interrupt` callback ABI on lua_Callbacks.
pub unsafe extern "C-unwind" fn sigint_callback(l: *mut lua_State, gc: c_int) {
    if gc >= 0 {
        return;
    }

    (*lua_callbacks(l)).interrupt = None;

    lua_rawcheckstack(l, 1); // reserve space for error string
    luaL_error!(l, "Execution interrupted");
}
