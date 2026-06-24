//! Faithful port of `runRepl` from Repl.cpp. Mirrors
//! `luau-repl-cli/src/functions/run_repl.rs`: create a fresh state, set it up,
//! arm Ctrl-C handling, sandbox the thread and run the interactive loop.

use core::sync::atomic::Ordering;

use luaur_vm::functions::lua_close::lua_close;
use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
use luaur_vm::functions::lua_l_sandboxthread::lua_l_sandboxthread;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::run_repl_impl::run_repl_impl;
use crate::functions::setup_state::setup_state;
use crate::functions::sigint_callback::REPL_STATE;

#[allow(non_snake_case)]
pub fn run_repl() {
    unsafe {
        let global_state = lua_l_newstate();
        let l: *mut lua_State = global_state;

        setup_state(l);

        // setup Ctrl+C handling: replState = L; signal(SIGINT, sigintHandler);
        REPL_STATE.store(l, Ordering::SeqCst);
        install_sigint_handler();

        lua_l_sandboxthread(l);
        run_repl_impl(l);

        // C++ wraps the state in a unique_ptr<lua_State, lua_close>; close it here.
        REPL_STATE.store(core::ptr::null_mut(), Ordering::SeqCst);
        lua_close(global_state);
    }
}

#[cfg(not(target_os = "windows"))]
unsafe fn install_sigint_handler() {
    use crate::functions::sigint_handler_repl::sigint_handler;

    // POSIX: signal(SIGINT, sigintHandler)
    const SIGINT: core::ffi::c_int = 2;
    extern "C" {
        fn signal(
            signum: core::ffi::c_int,
            handler: unsafe extern "C" fn(core::ffi::c_int),
        ) -> *mut core::ffi::c_void;
    }
    signal(SIGINT, sigint_handler);
}

#[cfg(target_os = "windows")]
unsafe fn install_sigint_handler() {
    use crate::functions::sigint_handler_repl_alt_b::sigint_handler;

    // Windows: SetConsoleCtrlHandler(sigintHandler, TRUE)
    extern "system" {
        fn SetConsoleCtrlHandler(
            handler: Option<unsafe extern "C" fn(u32) -> core::ffi::c_int>,
            add: core::ffi::c_int,
        ) -> core::ffi::c_int;
    }
    SetConsoleCtrlHandler(Some(sigint_handler), 1);
}
