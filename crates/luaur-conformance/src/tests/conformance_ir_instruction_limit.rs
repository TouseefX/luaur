//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:4370:conformance_ir_instruction_limit`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_ir_instruction_limit() {
    use crate::functions::run_conformance::CODEGEN;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use crate::type_aliases::state_ref::StateRef;
    use core::ffi::{c_char, c_void};
    use luaur_code_gen::enums::code_gen_compilation_result::CodeGenCompilationResult;
    use luaur_code_gen::enums::code_gen_flags::CodeGenFlags;
    use luaur_code_gen::functions::compile_internal::compile_internal;
    use luaur_code_gen::functions::luau_codegen_create::luau_codegen_create;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use luaur_code_gen::records::compilation_options::CompilationOptions;
    use luaur_code_gen::records::compilation_stats::CompilationStats;
    use luaur_common::FInt;
    use luaur_compiler::functions::luau_compile::luau_compile;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_l_openlibs::lua_l_openlibs;
    use luaur_vm::functions::lua_l_sandbox::lua_l_sandbox;
    use luaur_vm::functions::lua_l_sandboxthread::lua_l_sandboxthread;
    use luaur_vm::functions::luau_load::luau_load;
    use std::string::String;

    extern "C" {
        fn free(ptr: *mut c_void);
    }

    if unsafe { !CODEGEN } || luau_codegen_supported() == 0 {
        return;
    }

    let _codegen_heuristics_instruction_limit =
        ScopedFastInt::new(&FInt::CodegenHeuristicsInstructionLimit, 50_000);

    let mut source = String::new();

    for function_index in 0..100 {
        source.push_str("local function fn");
        source.push_str(&format!("{}", function_index));
        source.push_str("(...)\n");
        source.push_str("if ... then\n");
        source.push_str("local p1, p2 = ...\n");
        source.push_str("local _ = {\n");

        for i in 0..100 {
            source.push_str("p1*0.");
            source.push_str(&format!("{}", i));
            source.push_str(",");
            source.push_str("p2+0.");
            source.push_str(&format!("{}", i));
            source.push_str(",");
        }

        source.push_str("}\n");
        source.push_str("return _\n");
        source.push_str("end\n");
        source.push_str("end\n");
    }

    let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
        luau_codegen_create(l);

        lua_l_openlibs(l);
        lua_l_sandbox(l);
        lua_l_sandboxthread(l);

        let mut bytecode_size = 0usize;
        let bytecode = luau_compile(
            source.as_ptr() as *const c_char,
            source.len(),
            core::ptr::null_mut(),
            &mut bytecode_size,
        );
        assert!(!bytecode.is_null());

        let result = luau_load(l, c"=HugeFunction".as_ptr(), bytecode, bytecode_size, 0);
        free(bytecode as *mut c_void);

        assert_eq!(0, result);

        let mut native_options = CompilationOptions::default();
        native_options.flags = CodeGenFlags::CodeGen_ColdFunctions as u32;
        let mut native_stats = CompilationStats::default();
        let native_result = compile_internal(&None, l, -1, &native_options, &mut native_stats);

        assert_eq!(CodeGenCompilationResult::Success, native_result.result);
        assert!(native_result.has_errors());
        assert!(!native_result.proto_failures.is_empty());

        let first_failure = &native_result.proto_failures[0];
        assert_eq!(
            CodeGenCompilationResult::CodeGenOverflowInstructionLimit,
            first_failure.result
        );
        assert_ne!(-1, first_failure.line);
        assert_ne!("", first_failure.debugname);

        assert!(native_stats.functions_compiled > 0);
        assert!(native_stats.functions_compiled < 101);
    }
}
