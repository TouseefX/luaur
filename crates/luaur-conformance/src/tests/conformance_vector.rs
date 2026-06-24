//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:1700:conformance_vector`
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
//!   - calls -> function defaultOptions (tests/Conformance.test.cpp)
//!   - type_ref -> record CompilationOptions (CodeGen/include/Luau/CodeGenOptions.h)
//!   - calls -> function defaultCodegenOptions (tests/Conformance.test.cpp)
//!   - calls -> function vectorAccessBytecodeType (tests/ConformanceIrHooks.h)
//!   - calls -> function vectorNamecallBytecodeType (tests/ConformanceIrHooks.h)
//!   - calls -> function vectorAccess (tests/ConformanceIrHooks.h)
//!   - calls -> function vectorNamecall (tests/ConformanceIrHooks.h)
//!   - calls -> function runConformance (tests/Conformance.test.cpp)
//!   - calls -> function setupVectorHelpers (tests/Conformance.test.cpp)
//!   - calls -> function setupNativeHelpers (tests/Conformance.test.cpp)
//!   - translates_to -> rust_item conformance_vector

#[cfg(test)]
#[test]
fn conformance_vector() {
    use crate::functions::conformance_vector_setup::conformance_vector_setup;
    use crate::functions::default_codegen_options::default_codegen_options;
    use crate::functions::run_conformance::runConformance;
    use crate::methods::lowering_fixture_initialize_codegen::{
        vector_access_bytecode_type_callback, vector_access_callback,
        vector_namecall_bytecode_type_callback, vector_namecall_callback,
    };
    use luaur_compiler::records::lua_compile_options::LuaCompileOptions;

    for use_ir_hooks in [false, true] {
        for optimization_level in 0..=2 {
            let mut copts = LuaCompileOptions {
                optimization_level,
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
            };
            let mut native_options = default_codegen_options();

            if use_ir_hooks {
                native_options.hooks.vector_access_bytecode_type =
                    Some(vector_access_bytecode_type_callback);
                native_options.hooks.vector_namecall_bytecode_type =
                    Some(vector_namecall_bytecode_type_callback);
                native_options.hooks.vector_access = Some(vector_access_callback);
                native_options.hooks.vector_namecall = Some(vector_namecall_callback);
            }

            runConformance(
                c"vector.luau".as_ptr(),
                Some(conformance_vector_setup),
                None,
                core::ptr::null_mut(),
                &mut copts,
                false,
                &mut native_options,
            );
        }
    }
}
