//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:1665:conformance_c_yield`
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
//!   - calls -> function lua_pushcclosurek (VM/src/lapi.cpp)
//!   - calls -> function singleYield (tests/Conformance.test.cpp)
//!   - calls -> function singleYieldContinuation (tests/Conformance.test.cpp)
//!   - calls -> macro lua_setglobal (VM/include/lua.h)
//!   - calls -> function multipleYields (tests/Conformance.test.cpp)
//!   - calls -> function multipleYieldsContinuation (tests/Conformance.test.cpp)
//!   - calls -> function multipleYieldsWithNestedCall (tests/Conformance.test.cpp)
//!   - calls -> function multipleYieldsWithNestedCallContinuation (tests/Conformance.test.cpp)
//!   - calls -> function passthroughCall (tests/Conformance.test.cpp)
//!   - calls -> function passthroughCallContinuation (tests/Conformance.test.cpp)
//!   - calls -> function passthroughCallMoreResults (tests/Conformance.test.cpp)
//!   - calls -> function passthroughCallMoreResultsContinuation (tests/Conformance.test.cpp)
//!   - calls -> function passthroughCallArgReuse (tests/Conformance.test.cpp)
//!   - calls -> function passthroughCallArgReuseContinuation (tests/Conformance.test.cpp)
//!   - calls -> function passthroughCallVaradic (tests/Conformance.test.cpp)
//!   - calls -> function passthroughCallVaradicContinuation (tests/Conformance.test.cpp)
//!   - calls -> function passthroughCallWithState (tests/Conformance.test.cpp)
//!   - calls -> function passthroughCallWithStateContinuation (tests/Conformance.test.cpp)
//!   - translates_to -> rust_item conformance_c_yield

#[cfg(test)]
#[test]
fn conformance_c_yield() {
    use crate::functions::conformance_c_yield_setup::conformance_c_yield_setup;
    use crate::functions::run_conformance::runConformance;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _luau_resume_restore_c_calls = ScopedFastFlag::new(&FFlag::LuauResumeRestoreCcalls, true);

    runConformance(
        c"cyield.luau".as_ptr(),
        Some(conformance_c_yield_setup),
        None,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        false,
        core::ptr::null_mut(),
    );
}
