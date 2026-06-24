//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:1383:conformance_gc`
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
//!   - calls -> function luaL_checkboolean (VM/src/laux.cpp)
//!   - calls -> macro lua_setglobal (VM/include/lua.h)
//!   - calls -> function lua_newstate (VM/src/lstate.cpp)
//!   - calls -> function blockableRealloc (tests/Conformance.test.cpp)
//!   - translates_to -> rust_item conformance_gc

#[cfg(test)]
#[test]
fn conformance_gc() {
    use crate::functions::blockable_realloc::blockable_realloc;
    use crate::functions::conformance_gc_setup::conformance_gc_setup;
    use crate::functions::run_conformance::runConformance;
    use luaur_vm::functions::lua_newstate::lua_newstate;

    let initial_lua_state = unsafe { lua_newstate(Some(blockable_realloc), core::ptr::null_mut()) };

    runConformance(
        c"gc.luau".as_ptr(),
        Some(conformance_gc_setup),
        None,
        initial_lua_state,
        core::ptr::null_mut(),
        false,
        core::ptr::null_mut(),
    );
}
