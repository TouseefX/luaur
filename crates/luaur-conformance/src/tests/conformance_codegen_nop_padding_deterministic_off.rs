//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:4562:conformance_codegen_nop_padding_deterministic_off`
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
//!   - calls -> method FeedbackVectorFixture::compile (tests/FeedbackVector.test.cpp)
//!   - type_ref -> type_alias StateRef (tests/Conformance.test.cpp)
//!   - calls -> function luaL_newstate (VM/src/linit.cpp)
//!   - calls -> function lua_close (VM/src/lstate.cpp)
//!   - calls -> function luau_codegen_create (CodeGen/src/lcodegen.cpp)
//!   - calls -> function luau_compile (Compiler/src/lcode.cpp)
//!   - calls -> function luau_load (VM/src/lvmload.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> record CompilationStats (CodeGen/include/Luau/CodeGen.h)
//!   - type_ref -> record CompilationOptions (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item conformance_codegen_nop_padding_deterministic_off

#[cfg(test)]
#[test]
fn conformance_codegen_nop_padding_deterministic_off() {
    use crate::functions::run_conformance::CODEGEN;
    use crate::type_aliases::state_ref::StateRef;
    use core::ffi::{c_char, c_void};
    use luaur_code_gen::functions::compile_internal::compile_internal;
    use luaur_code_gen::functions::luau_codegen_create::luau_codegen_create;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use luaur_code_gen::records::compilation_options::CompilationOptions;
    use luaur_code_gen::records::compilation_stats::CompilationStats;
    use luaur_compiler::functions::luau_compile::luau_compile;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::luau_load::luau_load;

    extern "C" {
        fn free(ptr: *mut c_void);
    }

    if unsafe { !CODEGEN } || luau_codegen_supported() == 0 {
        return;
    }

    let source = r#"
        local function add(a, b) return a + b end
        return add(1, 2)
    "#;

    let compile = || -> usize {
        let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
        let l = global_state.as_ptr();

        unsafe {
            luau_codegen_create(l);

            let mut bytecode_size = 0usize;
            let bytecode = luau_compile(
                source.as_ptr() as *const c_char,
                source.len(),
                core::ptr::null_mut(),
                &mut bytecode_size,
            );
            assert!(!bytecode.is_null());

            let result = luau_load(l, c"=test".as_ptr(), bytecode, bytecode_size, 0);
            free(bytecode as *mut c_void);
            assert_eq!(0, result);

            let mut stats = CompilationStats::default();
            let options = CompilationOptions::default();
            let _ = compile_internal(&None, l, -1, &options, &mut stats);
            stats.native_code_size_bytes
        }
    };

    assert_eq!(compile(), compile());
}
