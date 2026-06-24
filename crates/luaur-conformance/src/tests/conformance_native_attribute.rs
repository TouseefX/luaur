//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:4508:conformance_native_attribute`
//! Source: `tests/Conformance.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Conformance.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file VM/include/lualib.h
//!   - includes -> source_file Compiler/include/luacode.h
//!   - includes -> source_file CodeGen/include/luacodegen.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
//!   - includes -> source_file CodeGen/include/Luau/BytecodeSummary.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file tests/ConformanceIrHooks.h
//! - incoming:
//!   - declares <- source_file tests/Conformance.test.cpp
//! - outgoing:
//!   - calls -> function luau_codegen_supported (CodeGen/src/lcodegen.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> type_alias StateRef (tests/Conformance.test.cpp)
//!   - calls -> function luaL_newstate (VM/src/linit.cpp)
//!   - calls -> function lua_close (VM/src/lstate.cpp)
//!   - calls -> function luau_codegen_create (CodeGen/src/lcodegen.cpp)
//!   - calls -> function luaL_openlibs (VM/src/linit.cpp)
//!   - calls -> function luaL_sandbox (VM/src/linit.cpp)
//!   - calls -> function luaL_sandboxthread (VM/src/linit.cpp)
//!   - calls -> function luau_compile (Compiler/src/lcode.cpp)
//!   - calls -> function luau_load (VM/src/lvmload.cpp)
//!   - type_ref -> enum Code (Config/include/Luau/LinterConfig.h)
//!   - type_ref -> record CompilationOptions (CodeGen/include/Luau/CodeGenOptions.h)
//!   - type_ref -> record CompilationStats (CodeGen/include/Luau/CodeGen.h)
//!   - type_ref -> record CompilationResult (CodeGen/include/Luau/CodeGen.h)
//!   - calls -> method FeedbackVectorFixture::compile (tests/FeedbackVector.test.cpp)
//!   - type_ref -> enum CodeGenCompilationResult (CodeGen/include/Luau/CodeGen.h)
//!   - translates_to -> rust_item conformance_native_attribute

#[cfg(test)]
#[test]
fn conformance_native_attribute() {
    use crate::functions::run_conformance::CODEGEN;
    use crate::type_aliases::state_ref::StateRef;
    use core::ffi::{c_char, c_void};
    use luaur_code_gen::enums::code_gen_compilation_result::CodeGenCompilationResult;
    use luaur_code_gen::enums::code_gen_flags::CodeGenFlags;
    use luaur_code_gen::functions::compile_internal::compile_internal;
    use luaur_code_gen::functions::luau_codegen_create::luau_codegen_create;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use luaur_code_gen::records::compilation_options::CompilationOptions;
    use luaur_code_gen::records::compilation_stats::CompilationStats;
    use luaur_compiler::functions::luau_compile::luau_compile;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_l_openlibs::lua_l_openlibs;
    use luaur_vm::functions::lua_l_sandbox::lua_l_sandbox;
    use luaur_vm::functions::lua_l_sandboxthread::lua_l_sandboxthread;
    use luaur_vm::functions::luau_load::luau_load;

    extern "C" {
        fn free(ptr: *mut c_void);
    }

    if unsafe { !CODEGEN } || luau_codegen_supported() == 0 {
        return;
    }

    let source = r#"
        @native
        local function sum(x, y)
            local function sumHelper(z)
                return (x+y+z)
            end
            return sumHelper
        end

        local function sub(x, y)
            @native
            local function subHelper(z)
                return (x+y-z)
            end
            return subHelper
        end"#;

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

        let result = luau_load(l, c"=Code".as_ptr(), bytecode, bytecode_size, 0);
        free(bytecode as *mut c_void);

        assert_eq!(0, result);

        let mut native_options = CompilationOptions::default();
        native_options.flags = CodeGenFlags::CodeGen_ColdFunctions as u32;
        let mut native_stats = CompilationStats::default();
        let native_result = compile_internal(&None, l, -1, &native_options, &mut native_stats);

        assert_eq!(CodeGenCompilationResult::Success, native_result.result);
        assert!(!native_result.has_errors());
        assert!(native_result.proto_failures.is_empty());

        assert_eq!(2, native_stats.functions_compiled);
    }
}
