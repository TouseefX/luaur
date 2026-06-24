//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:1863:conformance_types`
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
//!   - calls -> function runConformance (tests/Conformance.test.cpp)
//!   - type_ref -> record NullModuleResolver (Analysis/include/Luau/ModuleResolver.h)
//!   - type_ref -> record NullFileResolver (Analysis/include/Luau/FileResolver.h)
//!   - type_ref -> record Frontend (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
//!   - calls -> function registerBuiltinGlobals (Analysis/src/BuiltinDefinitions.cpp)
//!   - calls -> macro lua_newtable (VM/include/lua.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> function populateRTTI (tests/Conformance.test.cpp)
//!   - calls -> function lua_setfield (VM/src/lapi.cpp)
//!   - calls -> method Symbol::c_str (Analysis/include/Luau/Symbol.h)
//!   - calls -> macro lua_setglobal (VM/include/lua.h)
//!   - translates_to -> rust_item conformance_types

#[cfg(test)]
#[test]
fn conformance_types() {
    use crate::functions::conformance_types_setup::conformance_types_setup;
    use crate::functions::run_conformance::runConformance;

    runConformance(
        c"types.luau".as_ptr(),
        Some(conformance_types_setup),
        None,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        false,
        core::ptr::null_mut(),
    );
}
