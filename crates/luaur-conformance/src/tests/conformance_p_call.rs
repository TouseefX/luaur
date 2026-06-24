//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:1431:conformance_p_call`
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
//!   - calls -> function cxxthrow (tests/Conformance.test.cpp)
//!   - calls -> macro lua_setglobal (VM/include/lua.h)
//!   - calls -> function lua_tothread (VM/src/lapi.cpp)
//!   - calls -> function lua_xmove (VM/src/lapi.cpp)
//!   - calls -> function lua_resumeerror (VM/src/ldo.cpp)
//!   - calls -> function lua_newstate (VM/src/lstate.cpp)
//!   - calls -> function limitedRealloc (tests/Conformance.test.cpp)
//!   - translates_to -> rust_item conformance_p_call

#[cfg(test)]
#[test]
fn conformance_p_call() {
    use crate::functions::conformance_p_call_setup::conformance_p_call_setup;
    use crate::functions::limited_realloc::limited_realloc;
    use crate::functions::run_conformance::runConformance;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use luaur_vm::functions::lua_newstate::lua_newstate;

    let _luau_resume_restore_c_calls = ScopedFastFlag::new(&FFlag::LuauResumeRestoreCcalls, true);
    let initial_lua_state = unsafe { lua_newstate(Some(limited_realloc), core::ptr::null_mut()) };

    runConformance(
        c"pcall.luau".as_ptr(),
        Some(conformance_p_call_setup),
        None,
        initial_lua_state,
        core::ptr::null_mut(),
        false,
        core::ptr::null_mut(),
    );
}
