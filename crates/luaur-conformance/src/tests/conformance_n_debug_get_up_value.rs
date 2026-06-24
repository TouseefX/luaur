//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2203:conformance_n_debug_get_up_value`
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
//!   - calls -> function lua_checkstack (VM/src/lapi.cpp)
//!   - calls -> function lua_getinfo (VM/src/ldebug.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> macro upvalue (VM/src/lobject.h)
//!   - calls -> function lua_getupvalue (VM/src/lapi.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> macro lua_tointeger (VM/include/lua.h)
//!   - calls -> macro lua_pop (VM/include/lua.h)
//!   - translates_to -> rust_item conformance_n_debug_get_up_value

#[cfg(test)]
#[test]
fn conformance_n_debug_get_up_value() {
    use crate::functions::conformance_n_debug_get_up_value_yield::conformance_n_debug_get_up_value_yield;
    use crate::functions::run_conformance::runConformance;
    use luaur_compiler::records::lua_compile_options::LuaCompileOptions;

    let mut copts = LuaCompileOptions {
        optimization_level: 0,
        debug_level: 0,
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

    runConformance(
        c"ndebug_upvalues.luau".as_ptr(),
        None,
        Some(conformance_n_debug_get_up_value_yield),
        core::ptr::null_mut(),
        &mut copts,
        false,
        core::ptr::null_mut(),
    );
}
