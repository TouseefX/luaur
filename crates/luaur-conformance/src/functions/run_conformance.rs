use alloc::string::String;
use core::ffi::{c_char, c_int};
use std::ffi::{CStr, CString};

use crate::functions::default_codegen_options::default_codegen_options;
use crate::functions::find_conformance_source_dir::find_conformance_source_dir;
use crate::functions::lua_collectgarbage::lua_collectgarbage;
use crate::functions::lua_loadstring::lua_loadstring;
use crate::functions::lua_silence::lua_silence;
use crate::type_aliases::state_ref::StateRef;
use luaur_code_gen::enums::target::Target;
use luaur_code_gen::functions::get_assembly::get_assembly;
use luaur_code_gen::functions::luau_codegen_compile::luau_codegen_compile;
use luaur_code_gen::functions::luau_codegen_create::luau_codegen_create;
use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
use luaur_code_gen::records::assembly_options::AssemblyOptions;
use luaur_code_gen::records::compilation_options::CompilationOptions;
use luaur_code_gen::records::lowering_stats::{FunctionStats_Enable, LoweringStats};
use luaur_compiler::functions::luau_compile::luau_compile;
use luaur_compiler::records::lua_compile_options::LuaCompileOptions;
use luaur_vm::enums::lua_status::lua_Status;
use luaur_vm::functions::lua_c_validate::lua_c_validate;
use luaur_vm::functions::lua_debugtrace::lua_debugtrace;
use luaur_vm::functions::lua_isstring::lua_isstring;
use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
use luaur_vm::functions::lua_l_openlibs::lua_l_openlibs;
use luaur_vm::functions::lua_l_register::lua_l_register;
use luaur_vm::functions::lua_l_sandbox::lua_l_sandbox;
use luaur_vm::functions::lua_l_sandboxthread::lua_l_sandboxthread;
use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
use luaur_vm::functions::lua_resume::lua_resume;
use luaur_vm::functions::lua_resumeerror::lua_resumeerror;
use luaur_vm::functions::lua_setfield::lua_setfield;
use luaur_vm::functions::luau_load::luau_load;
use luaur_vm::macros::lua_globalsindex::LUA_GLOBALSINDEX;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_tostring::lua_tostring;
use luaur_vm::records::lua_l_reg::LuaLReg;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::type_aliases::lua_c_function::lua_CFunction;

extern "C" {
    fn free(ptr: *mut core::ffi::c_void);
}

pub static mut VERBOSE: bool = false;
pub static mut CODEGEN: bool = false;
pub static mut OPTIMIZATION_LEVEL: c_int = 1;

fn default_lua_compile_options() -> LuaCompileOptions {
    LuaCompileOptions {
        optimization_level: unsafe { OPTIMIZATION_LEVEL },
        debug_level: 1,
        type_info_level: 1,
        coverage_level: 0,
        vector_lib: core::ptr::null(),
        vector_ctor: core::ptr::null(),
        vector_type: core::ptr::null(),
        mutable_globals: core::ptr::null(),
        userdata_types: core::ptr::null(),
        libraries_with_known_members: core::ptr::null(),
        library_member_type_cb: None,
        library_member_constant_cb: None,
        disabled_builtins: core::ptr::null(),
    }
}

unsafe fn lua_c_function_from_safe(f: fn(*mut lua_State) -> c_int) -> lua_CFunction {
    Some(core::mem::transmute(f))
}

unsafe fn lua_c_function_from_extern(
    f: unsafe extern "C" fn(*mut lua_State) -> c_int,
) -> lua_CFunction {
    Some(core::mem::transmute(f))
}

#[allow(non_snake_case)]
pub fn runConformance(
    name: *const core::ffi::c_char,
    setup: Option<unsafe extern "C" fn(*mut lua_State)>,
    yield_fn: Option<unsafe extern "C" fn(*mut lua_State) -> bool>,
    initial_lua_state: *mut lua_State,
    options: *mut LuaCompileOptions,
    skip_codegen: bool,
    codegen_options: *mut CompilationOptions,
) -> StateRef {
    let name_str = unsafe { CStr::from_ptr(name) }
        .to_string_lossy()
        .into_owned();

    let mut path = std::env::var("LUAU_CONFORMANCE_SOURCE_DIR")
        .unwrap_or_else(|_| find_conformance_source_dir());
    if path.is_empty() {
        path = "Client/Luau/tests/conformance".to_owned();
    }
    if !path.ends_with('/') {
        path.push('/');
    }
    path.push_str(&name_str);

    let source = std::fs::read(&path).unwrap_or_else(|_| {
        panic!(
            "File {path} is not found. Make sure you run tests from the root or specify custom directory using LUAU_CONFORMANCE_SOURCE_DIR env variable"
        )
    });

    let initial_lua_state = if initial_lua_state.is_null() {
        lua_l_newstate()
    } else {
        initial_lua_state
    };

    let global_state = StateRef::new(initial_lua_state).expect("lua state allocation failed");
    let L = global_state.as_ptr();

    unsafe {
        if CODEGEN && !skip_codegen && luau_codegen_supported() != 0 {
            luau_codegen_create(L);
        }

        lua_l_openlibs(L);

        let mut funcs = vec![
            LuaLReg {
                name: c"collectgarbage".as_ptr(),
                func: lua_c_function_from_safe(lua_collectgarbage),
            },
            LuaLReg {
                name: c"loadstring".as_ptr(),
                func: lua_c_function_from_extern(lua_loadstring),
            },
        ];

        if !VERBOSE {
            funcs.push(LuaLReg {
                name: c"print".as_ptr(),
                func: Some(core::mem::transmute(
                    lua_silence as fn(*mut core::ffi::c_void) -> c_int,
                )),
            });
        }

        funcs.push(LuaLReg {
            name: core::ptr::null(),
            func: None,
        });

        lua_pushvalue(L, LUA_GLOBALSINDEX);
        lua_l_register(L, core::ptr::null(), funcs.as_ptr());
        lua_pop(L, 1);

        if let Some(setup_fn) = setup {
            setup_fn(L);
        }

        lua_l_sandbox(L);
        lua_l_sandboxthread(L);

        lua_pushvalue(L, LUA_GLOBALSINDEX);
        lua_setfield(L, -1, c"_G".as_ptr());

        let chunkname = CString::new(format!("={name_str}")).expect("chunk name contains nul");

        let mut local_options;
        let options = if options.is_null() {
            local_options = default_lua_compile_options();
            &mut local_options as *mut LuaCompileOptions
        } else {
            options
        };

        let mut bytecode_size = 0usize;
        let bytecode = luau_compile(
            source.as_ptr() as *const c_char,
            source.len(),
            options,
            &mut bytecode_size,
        );
        let load_result = luau_load(L, chunkname.as_ptr(), bytecode, bytecode_size, 0);
        free(bytecode as *mut core::ffi::c_void);

        let native_opts = if codegen_options.is_null() {
            default_codegen_options()
        } else {
            (*codegen_options).clone()
        };

        if load_result == 0 && CODEGEN && !skip_codegen && luau_codegen_supported() != 0 {
            luau_codegen_compile(L, -1);
        }

        if load_result == 0 && luau_codegen_supported() != 0 {
            let mut assembly_options = AssemblyOptions {
                target: Target::A64,
                compilation_options: native_opts.clone(),
                output_binary: false,
                include_assembly: true,
                include_ir: true,
                include_outlined_code: true,
                include_ir_types: true,
                include_ir_prefix: Default::default(),
                include_use_info: Default::default(),
                include_cfg_info: Default::default(),
                include_reg_flow_info: Default::default(),
                annotator: None,
                annotator_context: core::ptr::null_mut(),
            };
            let mut stats = LoweringStats::default();
            stats.function_stats_flags = FunctionStats_Enable;

            let a64 = get_assembly(L, -1, assembly_options.clone(), &mut stats);
            assert!(!a64.is_empty());
            assert_eq!(stats.reg_alloc_errors, 0);
            assert_eq!(stats.lowering_errors, 0);

            assembly_options.target = Target::X64_SystemV;
            let x64 = get_assembly(L, -1, assembly_options, &mut stats);
            assert!(!x64.is_empty());
            assert_eq!(stats.reg_alloc_errors, 0);
            assert_eq!(stats.lowering_errors, 0);
        }

        let mut status = if load_result == 0 {
            lua_resume(L, core::ptr::null_mut(), 0)
        } else {
            lua_Status::LUA_ERRSYNTAX as c_int
        };

        while let Some(yield_fn) = yield_fn {
            if status != lua_Status::LUA_YIELD as c_int && status != lua_Status::LUA_BREAK as c_int
            {
                break;
            }

            let resume_error = yield_fn(L);
            status = if resume_error {
                lua_resumeerror(L, core::ptr::null_mut())
            } else {
                lua_resume(L, core::ptr::null_mut(), 0)
            };
        }

        lua_c_validate(L);

        if status == 0 {
            assert!(lua_isstring(L, -1) != 0);
            let result = CStr::from_ptr(lua_tostring!(L, -1)).to_string_lossy();
            assert_eq!(result.as_ref(), "OK");
            lua_pop(L, 1);
        } else {
            let error = if status == lua_Status::LUA_YIELD as c_int {
                String::from("thread yielded unexpectedly")
            } else {
                CStr::from_ptr(lua_tostring!(L, -1))
                    .to_string_lossy()
                    .into_owned()
            };
            let trace = CStr::from_ptr(lua_debugtrace(L)).to_string_lossy();
            panic!("{error}\nstacktrace:\n{trace}");
        }
    }

    global_state
}
