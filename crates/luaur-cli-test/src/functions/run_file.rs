//! Node: `cxx:Function:Luau.CLI.Test:CLI/src/Repl.cpp:572:run_file`
//! Source: `CLI/src/Repl.cpp:572-647` (faithful port)
//!
//! `repl` indicates whether a REPL should be started after executing the file.
//! No CLI-test invocation passes `-i`, so the `runReplImpl` branch is never
//! reached from these tests.

use alloc::string::String;
use core::ffi::{c_char, c_void, CStr};

use luaur_cli_lib::functions::normalize_path::normalize_path;
use luaur_cli_lib::functions::read_file::read_file;

use luaur_compiler::functions::luau_compile::luau_compile;
use luaur_compiler::records::lua_compile_options::LuaCompileOptions;

use luaur_vm::functions::lua_debugtrace::lua_debugtrace;
use luaur_vm::functions::lua_l_sandboxthread::lua_l_sandboxthread;
use luaur_vm::functions::lua_newthread::lua_newthread;
use luaur_vm::functions::lua_resume::lua_resume;
use luaur_vm::functions::lua_tolstring::lua_tolstring;
use luaur_vm::functions::luau_load::luau_load;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::copts::copts;
use crate::functions::repl_main::{program_argc, program_argv};
use crate::functions::setup_arguments::setup_arguments;

// Status codes from VM/include/lua.h.
const LUA_YIELD: i32 = 1;
const LUA_ERRSYNTAX: i32 = 3;

extern "C" {
    fn free(ptr: *mut c_void);
}

/// Faithful port of the C++ `runFile`.
pub fn run_file(name: *const c_char, gl: *mut lua_State, repl: bool) -> bool {
    unsafe {
        let name_str = CStr::from_ptr(name).to_string_lossy().into_owned();

        let source = match read_file(&name_str) {
            Some(s) => s,
            None => {
                eprint!("Error opening {}\n", name_str);
                return false;
            }
        };

        // module needs to run in a new thread, isolated from the rest
        let l = lua_newthread(gl as *mut _);

        // new thread needs to have the globals sandboxed
        lua_l_sandboxthread(l);

        // chunkname = "@" + normalizePath(name)  (NUL-terminated for the C ABI)
        let chunkname = format!("@{}\0", normalize_path(&name_str));

        // Luau::compile(*source, copts()) -> bytecode string.
        let mut options: LuaCompileOptions = copts();
        let mut bytecode_size: usize = 0;
        let bytecode = luau_compile(
            source.as_ptr() as *const c_char,
            source.len(),
            &mut options as *mut LuaCompileOptions,
            &mut bytecode_size,
        );

        let mut status: i32 = 0;

        let load_status = luau_load(
            l,
            chunkname.as_ptr() as *const c_char,
            bytecode as *const c_char,
            bytecode_size,
            0,
        );

        free(bytecode as *mut c_void);

        if load_status == 0 {
            // codegen / coverage / counters are inactive in the CLI-test harness
            // (no --codegen flag, no coverageInit/countersInit), so the upstream
            // CodeGen::compile / coverageTrack / countersTrack branches are not
            // taken here.

            setup_arguments(l as *mut _, program_argc(), program_argv());
            status = lua_resume(l, core::ptr::null_mut(), program_argc());
        } else {
            status = LUA_ERRSYNTAX;
        }

        if status != 0 {
            let mut error = String::new();

            if status == LUA_YIELD {
                error.push_str("thread yielded unexpectedly");
            } else {
                let str_ptr = lua_tolstring(l, -1, core::ptr::null_mut());
                if !str_ptr.is_null() {
                    error.push_str(&CStr::from_ptr(str_ptr).to_string_lossy());
                }
            }

            error.push_str("\nstacktrace:\n");
            let trace = lua_debugtrace(l);
            if !trace.is_null() {
                error.push_str(&CStr::from_ptr(trace).to_string_lossy());
            }

            eprint!("{}", error);
        }

        if repl {
            // Upstream calls runReplImpl(L) here. No CLI-test invocation passes
            // -i, so this branch is unreachable on the tested path.
            panic!(
                "interactive REPL after file execution is not supported in the CLI-test harness"
            );
        }

        lua_pop(gl, 1);
        status == 0
    }
}
