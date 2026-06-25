//! Windows variant of Repl.cpp's `sigintHandler`. Mirrors
//! `luau-repl-cli/src/functions/sigint_handler_repl_alt_b.rs`.
//!
//! ```c++
//!     BOOL WINAPI sigintHandler(DWORD signal)
//!     {
//!         if (signal == CTRL_C_EVENT && replState)
//!             lua_callbacks(replState)->interrupt = &sigintCallback;
//!         return TRUE;
//!     }
//! ```
//!
//! Registered via `SetConsoleCtrlHandler`; returning TRUE (1) tells Windows the
//! event was handled. Arms the same interrupt callback as the POSIX variant.

use core::ffi::c_int;
use core::sync::atomic::Ordering;

use luaur_vm::functions::lua_callbacks::lua_callbacks;

use crate::functions::sigint_callback::{sigint_callback, REPL_STATE};

// Windows console control event for Ctrl-C (CTRL_C_EVENT).
const CTRL_C_EVENT: u32 = 0;

pub unsafe extern "C" fn sigint_handler(signal: u32) -> c_int {
    let repl_state = REPL_STATE.load(Ordering::SeqCst);
    if signal == CTRL_C_EVENT && !repl_state.is_null() {
        (*lua_callbacks(repl_state)).interrupt = Some(sigint_callback);
    }
    1 // TRUE
}
