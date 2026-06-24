//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3076:conformance_coverage`
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
//!   - calls -> function runConformance (tests/Conformance.test.cpp)
//!   - calls -> macro lua_pushcfunction (VM/include/lua.h)
//!   - calls -> macro luaL_argexpected (VM/include/lualib.h)
//!   - calls -> function lua_isLfunction (VM/src/lapi.cpp)
//!   - calls -> macro lua_newtable (VM/include/lua.h)
//!   - calls -> function lua_getcoverage (VM/src/ldebug.cpp)
//!   - calls -> function lua_pushstring (VM/src/lapi.cpp)
//!   - calls -> function lua_setfield (VM/src/lapi.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> function lua_pushinteger (VM/src/lapi.cpp)
//!   - calls -> function lua_rawseti (VM/src/lapi.cpp)
//!   - calls -> function lua_objlen (VM/src/lapi.cpp)
//!   - calls -> function getcoverage (VM/src/ldebug.cpp)
//!   - calls -> macro lua_setglobal (VM/include/lua.h)
//!   - translates_to -> rust_item conformance_coverage

#[cfg(test)]
#[test]
fn conformance_coverage() {
    use crate::functions::conformance_coverage_setup::conformance_coverage_setup;
    use crate::functions::run_conformance::runConformance;
    use luaur_compiler::records::lua_compile_options::LuaCompileOptions;

    let mut copts = LuaCompileOptions {
        optimization_level: 1,
        debug_level: 1,
        type_info_level: 1,
        coverage_level: 2,
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

    runConformance(
        c"coverage.luau".as_ptr(),
        Some(conformance_coverage_setup),
        None,
        core::ptr::null_mut(),
        &mut copts,
        false,
        core::ptr::null_mut(),
    );
}
