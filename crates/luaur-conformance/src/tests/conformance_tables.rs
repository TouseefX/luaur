//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:1260:conformance_tables`
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
//!   - calls -> macro lua_pushcfunction (VM/include/lua.h)
//!   - calls -> function lua_type (VM/src/lapi.cpp)
//!   - calls -> function luaL_checkunsigned (VM/src/laux.cpp)
//!   - calls -> macro lua_pushlightuserdata (VM/include/lua.h)
//!   - calls -> function lua_topointer (VM/src/lapi.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> macro lua_setglobal (VM/include/lua.h)
//!   - translates_to -> rust_item conformance_tables

#[cfg(test)]
#[test]
fn conformance_tables() {
    use crate::functions::conformance_tables_setup::conformance_tables_setup;
    use crate::functions::run_conformance::runConformance;

    runConformance(
        c"tables.luau".as_ptr(),
        Some(conformance_tables_setup),
        None,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        false,
        core::ptr::null_mut(),
    );
}
