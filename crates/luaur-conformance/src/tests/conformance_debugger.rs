//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:1899:conformance_debugger`
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
//!   - calls -> function lua_callbacks (VM/src/lapi.cpp)
//!   - calls -> function lua_singlestep (VM/src/ldebug.cpp)
//!   - calls -> function lua_debugtrace (VM/src/ldebug.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> function lua_break (VM/src/ldo.cpp)
//!   - calls -> function resume (VM/src/ldo.cpp)
//!   - calls -> function lua_pushcclosurek (VM/src/lapi.cpp)
//!   - calls -> function luaL_checkinteger (VM/src/laux.cpp)
//!   - calls -> function luaL_optboolean (VM/src/laux.cpp)
//!   - calls -> function lua_getinfo (VM/src/ldebug.cpp)
//!   - calls -> function lua_stackdepth (VM/src/ldebug.cpp)
//!   - calls -> function lua_breakpoint (VM/src/ldebug.cpp)
//!   - calls -> macro lua_setglobal (VM/include/lua.h)
//!   - calls -> function lua_checkstack (VM/src/lapi.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function lua_getargument (VM/src/ldebug.cpp)
//!   - calls -> macro lua_tointeger (VM/include/lua.h)
//!   - calls -> macro lua_pop (VM/include/lua.h)
//!   - calls -> function lua_getlocal (VM/src/ldebug.cpp)
//!   - calls -> function lua_getupvalue (VM/src/lapi.cpp)
//!   - calls -> method BytecodeBuilder::validate (Bytecode/src/BytecodeBuilder.cpp)
//!   - calls -> macro lua_isnil (VM/include/lua.h)
//!   - calls -> function lua_resume (VM/src/ldo.cpp)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - translates_to -> rust_item conformance_debugger

#[cfg(test)]
#[test]
fn conformance_debugger() {
    use crate::functions::conformance_debugger_setup::conformance_debugger_setup;
    use crate::functions::conformance_debugger_yield::conformance_debugger_yield;
    use crate::functions::run_conformance::runConformance;
    use crate::records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE;
    use core::sync::atomic::Ordering;
    use luaur_compiler::records::lua_compile_options::LuaCompileOptions;

    for singlestep in [false, true] {
        CONFORMANCE_DEBUGGER_STATE.reset(singlestep);

        let mut copts = LuaCompileOptions {
            optimization_level: 1,
            debug_level: 2,
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
            c"debugger.luau".as_ptr(),
            Some(conformance_debugger_setup),
            Some(conformance_debugger_yield),
            core::ptr::null_mut(),
            &mut copts,
            true,
            core::ptr::null_mut(),
        );

        assert_eq!(
            CONFORMANCE_DEBUGGER_STATE.breakhits.load(Ordering::SeqCst),
            16
        );

        if singlestep {
            assert!(CONFORMANCE_DEBUGGER_STATE.stephits.load(Ordering::SeqCst) > 100);
        }
    }
}
