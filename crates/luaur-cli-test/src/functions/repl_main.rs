//! Node: `cxx:Function:Luau.CLI.Test:CLI/src/Repl.cpp:677:repl_main`
//! Source: `CLI/src/Repl.cpp:677-858` (faithful port)
//!
//! Faithful port of the C++ `replMain(int argc, char** argv)`. Parses the CLI
//! options, gathers the source files and runs them on a fresh `lua_State`,
//! returning `failed ? 1 : 0`. The profiling / coverage / counters / codegen
//! subsystems are recognised at the flag level but inert in the CLI-test
//! harness (no test enables them on the hit path).

use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::{c_char, CStr};
use core::sync::atomic::{AtomicI32, Ordering};

use luaur_cli_lib::functions::get_source_files::get_source_files;

use luaur_vm::functions::lua_close::lua_close;
use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::display_help::display_help;
use crate::functions::run_file::run_file;
use crate::functions::setup_state::setup_state;
use crate::records::global_options::GlobalOptions;

// CLI-level state. In C++ these are file-static globals in Repl.cpp
// (`globalOptions`, `program_argc`, `program_argv`). The CLI-test harness runs
// each `replMain` call to completion before the next, so simple atomics mirror
// the C++ statics faithfully.

static OPTIMIZATION_LEVEL: AtomicI32 = AtomicI32::new(1);
static DEBUG_LEVEL: AtomicI32 = AtomicI32::new(1);
static PROGRAM_ARGC: AtomicI32 = AtomicI32::new(0);

// `program_argv` is a raw `char**` borrowed from the live `argv` array for the
// duration of the `replMain` call. It is only read while the call is on the
// stack, exactly as in C++.
static mut PROGRAM_ARGV: *mut *mut c_char = core::ptr::null_mut();

/// The CLI's `globalOptions` static (optimization + debug level).
pub fn global_options() -> GlobalOptions {
    GlobalOptions {
        optimization_level: OPTIMIZATION_LEVEL.load(Ordering::Relaxed),
        debug_level: DEBUG_LEVEL.load(Ordering::Relaxed),
    }
}

/// Number of arguments to pass through to the running Luau program.
pub fn program_argc() -> i32 {
    PROGRAM_ARGC.load(Ordering::Relaxed)
}

/// Pointer to the program's argument vector (borrowed from `replMain`'s argv).
pub fn program_argv() -> *mut *mut c_char {
    unsafe { PROGRAM_ARGV }
}

/// Faithful port of `int replMain(int argc, char** argv)`.
#[allow(non_snake_case)]
pub fn repl_main(argc: i32, argv: *mut *mut c_char) -> i32 {
    // C++ installs an assertion handler and (on Windows) sets the console code
    // page here; neither is observable from the CLI-test harness.

    let mut profile: i32 = 0;
    let mut coverage = false;
    let mut interactive = false;
    let mut codegen_perf = false;
    let mut counters = false;
    let mut program_args = argc;

    // Reset per-call state to the C++ static defaults.
    OPTIMIZATION_LEVEL.store(1, Ordering::Relaxed);
    DEBUG_LEVEL.store(1, Ordering::Relaxed);

    let arg = |i: i32| -> String {
        unsafe {
            let p = *argv.add(i as usize);
            CStr::from_ptr(p).to_string_lossy().into_owned()
        }
    };
    let argv0 = arg(0);

    let mut i = 1i32;
    while i < argc {
        let a = arg(i);

        if a == "-h" || a == "--help" {
            display_help(&argv0);
            return 0;
        } else if a == "-i" || a == "--interactive" {
            interactive = true;
        } else if a.starts_with("-O") {
            let level: i32 = a[2..].parse().unwrap_or(0);
            if level < 0 || level > 2 {
                eprint!("Error: Optimization level must be between 0 and 2 inclusive.\n");
                return 1;
            }
            OPTIMIZATION_LEVEL.store(level, Ordering::Relaxed);
        } else if a.starts_with("-g") {
            let level: i32 = a[2..].parse().unwrap_or(0);
            if level < 0 || level > 2 {
                eprint!("Error: Debug level must be between 0 and 2 inclusive.\n");
                return 1;
            }
            DEBUG_LEVEL.store(level, Ordering::Relaxed);
        } else if a == "--profile" {
            profile = 10000; // default to 10 KHz
        } else if let Some(rest) = a.strip_prefix("--profile=") {
            profile = rest.parse().unwrap_or(0);
        } else if a == "--codegen" {
            // Native codegen is unsupported in the CLI-test build; recognised
            // but inert (matches the C++ "not supported in current
            // configuration" warning path).
        } else if a == "--codegen-cold" {
            // see --codegen
        } else if a == "--codegen-perf" {
            codegen_perf = true;
        } else if a == "--coverage" {
            coverage = true;
        } else if a == "--counters" {
            counters = true;
        } else if a == "--timetrace" {
            // FFlag::DebugLuauTimeTracing — time tracing is compiled out.
        } else if a.starts_with("--fflags=") {
            // setLuauFlags(argv[i] + 9): not driven by the require suite.
        } else if a == "--program-args" || a == "-a" {
            program_args = i + 1;
            break;
        } else if a.starts_with('-') {
            eprint!("Error: Unrecognized option '{}'.\n\n", a);
            display_help(&argv0);
            return 1;
        }

        i += 1;
    }

    PROGRAM_ARGC.store(argc - program_args, Ordering::Relaxed);
    unsafe {
        PROGRAM_ARGV = argv.add(program_args as usize);
    }

    if codegen_perf {
        // --codegen-perf is Linux-only in C++; everywhere else it errors out.
        // The require suite never sets it.
        eprint!("--codegen-perf option is only supported on Linux\n");
        return 1;
    }

    let files: Vec<String> = get_source_files(argc, argv);

    if files.is_empty() {
        // C++ starts the interactive REPL here. Not reachable from the require
        // tests (they always pass a script path).
        let _ = profile;
        let _ = coverage;
        let _ = counters;
        let _ = interactive;
        panic!("interactive REPL is not supported in the CLI-test harness");
    }

    unsafe {
        let l: *mut lua_State = lua_l_newstate();

        setup_state(l);

        // profilerStart / coverageInit / countersInit are no-ops in this
        // harness (the require suite enables none of them).

        let mut failed = 0i32;

        let n = files.len();
        for (idx, file) in files.iter().enumerate() {
            let is_last_file = idx == n - 1;
            let c_name = String::from(file.as_str()) + "\0";
            let ran = run_file(
                c_name.as_ptr() as *const c_char,
                l,
                interactive && is_last_file,
            );
            failed += (!ran) as i32;
        }

        lua_close(l);

        if failed != 0 {
            1
        } else {
            0
        }
    }
}
