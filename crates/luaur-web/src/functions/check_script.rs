//! `extern "C" const char* checkScript(const char* source, int useNewSolver)`
//! (`CLI/src/Web.cpp:142-182`).
//!
//! The wasm type-checking entry point: builds a demo `Frontend`, registers the
//! Luau builtins, type-checks the single module `"main"`, and returns the
//! newline-joined `line: message` diagnostics (or null when clean). The C++
//! caches the result in a function-`static std::string` so the returned pointer
//! outlives the call; the Rust analog is a thread-local `CString`.

use crate::records::demo_config_resolver::DemoConfigResolver;
use crate::records::demo_file_resolver::DemoFileResolver;
use alloc::string::{String, ToString};
use core::cell::RefCell;
use core::ffi::{c_char, c_int, CStr};
use std::ffi::CString;

use luaur_analysis::enums::solver_mode::SolverMode;
use luaur_analysis::functions::freeze::freeze;
use luaur_analysis::functions::register_builtin_globals::register_builtin_globals;
use luaur_analysis::functions::to_string_error::to_string_type_error;
use luaur_analysis::functions::unfreeze::unfreeze;
use luaur_analysis::records::frontend::Frontend;
use luaur_analysis::records::frontend_options::FrontendOptions;

thread_local! {
    /// Mirror of the C++ `static std::string finalCheckResult;` — keeps the
    /// returned C string alive after the call returns.
    static FINAL_CHECK_RESULT: RefCell<Option<CString>> = const { RefCell::new(None) };
}

/// The fallible body, run under `catch_unwind` to reproduce the C++
/// `try { ... } catch (const std::exception& e) { finalCheckResult = e.what(); }`.
fn run_check(source: &str, use_new_solver: c_int) -> String {
    let mut final_check_result = String::new();

    let mut file_resolver = DemoFileResolver::new();
    let mut config_resolver = DemoConfigResolver::demo_config_resolver();
    let options = FrontendOptions::default();

    let mut frontend = Frontend::frontend_file_resolver_config_resolver_frontend_options(
        &mut file_resolver.base,
        &mut config_resolver.base,
        &options,
    );
    unsafe {
        frontend.wire_self_pointers();
    }

    frontend.set_luau_solver_mode(if use_new_solver != 0 {
        SolverMode::New
    } else {
        SolverMode::Old
    });

    // Add Luau builtins:
    //   Luau::unfreeze(frontend.globals.globalTypes);
    //   Luau::registerBuiltinGlobals(frontend, frontend.globals);
    //   Luau::freeze(frontend.globals.globalTypes);
    let frontend_ptr = &mut frontend as *mut Frontend;
    unsafe {
        unfreeze((*frontend_ptr).globals.global_types_mut());
        register_builtin_globals(&mut *frontend_ptr, &mut (*frontend_ptr).globals, false);
        freeze((*frontend_ptr).globals.global_types_mut());
    }

    // restart
    //   frontend.clear();
    //   fileResolver.source.clear();
    frontend.clear();
    file_resolver.source.clear();

    // fileResolver.source["main"] = source;
    file_resolver
        .source
        .insert("main".to_string(), source.to_string());

    // Luau::CheckResult checkResult = frontend.check("main");
    let check_result =
        frontend.check_module_name_optional_frontend_options(&"main".to_string(), None);

    for err in &check_result.errors {
        if !final_check_result.is_empty() {
            final_check_result.push('\n');
        }
        // std::to_string(err.location.begin.line + 1)
        final_check_result.push_str(&(err.location.begin.line + 1).to_string());
        final_check_result.push_str(": ");
        // Luau::toString(err)
        final_check_result.push_str(&to_string_type_error(err));
    }

    final_check_result
}

/// # Safety
/// `source` must be a valid, NUL-terminated C string (the wasm/JS caller's
/// contract), or null.
#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn check_script(
    source: *const c_char,
    use_new_solver: c_int,
) -> *const c_char {
    let source_str = if source.is_null() {
        String::new()
    } else {
        core::str::from_utf8_unchecked(CStr::from_ptr(source).to_bytes()).to_string()
    };

    // try { ... } catch (const std::exception& e) { finalCheckResult = e.what(); }
    let final_check_result =
        match std::panic::catch_unwind(move || run_check(&source_str, use_new_solver)) {
            Ok(result) => result,
            Err(payload) => panic_message(&payload),
        };

    if final_check_result.is_empty() {
        FINAL_CHECK_RESULT.with(|r| *r.borrow_mut() = None);
        return core::ptr::null();
    }

    FINAL_CHECK_RESULT.with(|r| {
        let cstring = CString::new(final_check_result).unwrap_or_default();
        let ptr = cstring.as_ptr();
        *r.borrow_mut() = Some(cstring);
        ptr
    })
}

/// Extract a `std::exception::what()`-equivalent message from a caught panic
/// payload.
fn panic_message(payload: &(dyn core::any::Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "unknown error".to_string()
    }
}
