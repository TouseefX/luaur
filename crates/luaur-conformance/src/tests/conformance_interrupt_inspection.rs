//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2092:conformance_interrupt_inspection`
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
//!   - calls -> function lua_callbacks (VM/src/lapi.cpp)
//!   - calls -> function lua_isyieldable (VM/src/ldo.cpp)
//!   - calls -> function lua_break (VM/src/ldo.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> function lua_getinfo (VM/src/ldebug.cpp)
//!   - calls -> function luau_callhook (VM/src/lvmexecute.cpp)
//!   - translates_to -> rust_item conformance_interrupt_inspection

#[cfg(test)]
#[test]
fn conformance_interrupt_inspection() {
    use crate::functions::conformance_interrupt_inspection_setup::conformance_interrupt_inspection_setup;
    use crate::functions::conformance_interrupt_inspection_yield::conformance_interrupt_inspection_yield;
    use crate::functions::run_conformance::runConformance;

    runConformance(
        c"basic.luau".as_ptr(),
        Some(conformance_interrupt_inspection_setup),
        Some(conformance_interrupt_inspection_yield),
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        true,
        core::ptr::null_mut(),
    );
}
