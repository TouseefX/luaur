use core::ffi::c_int;
use core::sync::atomic::Ordering;

use luaur_vm::functions::lua_callbacks::lua_callbacks;

use crate::functions::sigint_callback::{sigint_callback, REPL_STATE};

// SIGINT value on POSIX systems.
const SIGINT: c_int = 2;

// Unix variant of Repl.cpp's `sigintHandler`:
//
//     static void sigintHandler(int signum)
//     {
//         if (signum == SIGINT && replState)
//             lua_callbacks(replState)->interrupt = &sigintCallback;
//     }
//
// Installed with `signal(SIGINT, sigintHandler)`; it merely arms the interrupt
// callback so the VM raises "Execution interrupted" at the next safe point.
pub unsafe extern "C" fn sigint_handler(signum: c_int) {
    let repl_state = REPL_STATE.load(Ordering::SeqCst);
    if signum == SIGINT && !repl_state.is_null() {
        (*lua_callbacks(repl_state)).interrupt = Some(sigint_callback);
    }
}
