//! `extern "C" const char* executeScript(const char* source)`
//! (`CLI/src/Web.cpp:184-208`).
//!
//! The wasm entry point: enables every `Luau*` bool fast flag, spins up a fresh
//! sandboxed Lua state, runs the script via [`run_code`], and returns the result
//! string (or null when empty). The C++ caches the result in a function-`static`
//! `std::string` so the returned pointer outlives the call; the Rust analog is a
//! thread-local `CString` whose pointer is returned.

use crate::functions::run_code::run_code;
use crate::functions::setup_state::setup_state;
use core::cell::RefCell;
use core::ffi::{c_char, CStr};
use std::ffi::CString;

use luaur_common::set_luau_bool_flags;
use luaur_vm::functions::lua_close::lua_close;
use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
use luaur_vm::functions::lua_l_sandboxthread::lua_l_sandboxthread;
use luaur_vm::type_aliases::lua_state::lua_State;

thread_local! {
    /// Mirror of the C++ `static std::string result;` — keeps the returned
    /// C string alive after the call returns.
    static RESULT: RefCell<Option<CString>> = const { RefCell::new(None) };
}

/// # Safety
/// `source` must be a valid, NUL-terminated C string (the wasm/JS caller's
/// contract), or null.
#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn execute_script(source: *const c_char) -> *const c_char {
    // setup flags:
    //   for (FValue<bool>* flag = FValue<bool>::list; flag; flag = flag->next)
    //       if (strncmp(flag->name, "Luau", 4) == 0)
    //           flag->value = true;
    //
    // In this port the per-type intrusive `FValue<bool>::list` is never
    // populated (Rust statics cannot self-register in a ctor, and no startup
    // `register()` runs), so the C++ name-prefix walk is expressed through the
    // crate's public flag-enabling analog `set_luau_bool_flags`, which turns on
    // every non-`Debug` bool FastFlag (the `Luau*` flags dominate that set).
    set_luau_bool_flags(true);

    // create new state: unique_ptr<lua_State, lua_close> globalState(luaL_newstate(), lua_close);
    let l: *mut lua_State = lua_l_newstate();

    // setup state
    setup_state(l);

    // sandbox thread
    lua_l_sandboxthread(l);

    // run code + collect error
    let source_str = if source.is_null() {
        ""
    } else {
        core::str::from_utf8_unchecked(CStr::from_ptr(source).to_bytes())
    };
    let result = run_code(l, source_str);

    // unique_ptr destructor: lua_close(L)
    lua_close(l);

    if result.is_empty() {
        // cache empty (drop any prior held string) and return nullptr.
        RESULT.with(|r| *r.borrow_mut() = None);
        return core::ptr::null();
    }

    RESULT.with(|r| {
        let cstring = CString::new(result).unwrap_or_default();
        let ptr = cstring.as_ptr();
        *r.borrow_mut() = Some(cstring);
        ptr
    })
}
