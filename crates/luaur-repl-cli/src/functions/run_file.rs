use alloc::string::String;
use core::ffi::{c_char, CStr};

use luaur_ast::records::parse_options::ParseOptions;
use luaur_bytecode::records::bytecode_encoder::BytecodeEncoder;
use luaur_cli_lib::functions::normalize_path::normalize_path;
use luaur_cli_lib::functions::read_file::read_file;
use luaur_code_gen::functions::luau_codegen_compile::luau_codegen_compile;
use luaur_compiler::functions::compile::compile;
use luaur_vm::enums::lua_status::lua_Status;
use luaur_vm::functions::lua_debugtrace::lua_debugtrace;
use luaur_vm::functions::lua_l_sandboxthread::lua_l_sandboxthread;
use luaur_vm::functions::lua_newthread::lua_newthread;
use luaur_vm::functions::lua_resume::lua_resume;
use luaur_vm::functions::luau_load::luau_load;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_tostring::lua_tostring;
use luaur_vm::type_aliases::lua_state::lua_State;

use crate::functions::copts::copts;
use crate::functions::counters_active::counters_active;
use crate::functions::counters_track::counters_track;
use crate::functions::coverage_active::coverage_active;
use crate::functions::coverage_track::coverage_track;
use crate::functions::repl_main::{program_argc, program_argv, repl_codegen_enabled};
use crate::functions::run_repl_impl::run_repl_impl;
use crate::functions::setup_arguments::setup_arguments;

// `repl` is used to indicate if a repl should be started after executing the file.
pub unsafe fn run_file(name: &str, gl: *mut lua_State, repl: bool) -> bool {
    let source = read_file(name);
    let source = match source {
        Some(s) => s,
        None => {
            eprintln!("Error opening {}", name);
            return false;
        }
    };

    // module needs to run in a new thread, isolated from the rest
    let l = lua_newthread(gl);

    // new thread needs to have the globals sandboxed
    lua_l_sandboxthread(l);

    let chunkname = String::from("@") + &normalize_path(name) + "\0";

    struct NoopEncoder;
    impl BytecodeEncoder for NoopEncoder {
        fn encode(&mut self, _data: &mut [u32]) {}
    }
    let options = copts();
    let parse_options = ParseOptions::default();
    let mut encoder = NoopEncoder;
    let bytecode = compile(
        &source,
        &options,
        &parse_options,
        &mut encoder as *mut dyn BytecodeEncoder,
    );

    let status: i32 = if luau_load(
        l,
        chunkname.as_ptr() as *const c_char,
        bytecode.as_ptr() as *const c_char,
        bytecode.len(),
        0,
    ) == 0
    {
        if repl_codegen_enabled() {
            // C++ sets CodeGen_ColdFunctions / recordCounters on the
            // CompilationOptions; the public Rust codegen API takes neither.
            luau_codegen_compile(l, -1);
        }

        if coverage_active() {
            coverage_track(l, -1);
        }

        if counters_active() {
            counters_track(l, -1);
        }

        setup_arguments(l, program_argc(), program_argv());
        lua_resume(l, core::ptr::null_mut(), program_argc())
    } else {
        lua_Status::LUA_ERRSYNTAX as i32
    };

    if status != 0 {
        let mut error: String;

        if status == lua_Status::LUA_YIELD as i32 {
            error = "thread yielded unexpectedly".into();
        } else {
            let str_ptr = lua_tostring!(l, -1);
            if !str_ptr.is_null() {
                error = CStr::from_ptr(str_ptr).to_string_lossy().into_owned();
            } else {
                error = String::new();
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
        run_repl_impl(l);
    }
    lua_pop(gl, 1);
    status == 0
}
