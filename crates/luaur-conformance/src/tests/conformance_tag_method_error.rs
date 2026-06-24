//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3022:conformance_tag_method_error`
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
//!   - type_ref -> record Loop (Compiler/src/Compiler.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function lua_break (VM/src/ldo.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> function lua_resumeerror (VM/src/ldo.cpp)
//!   - calls -> function runConformance (tests/Conformance.test.cpp)
//!   - calls -> function lua_callbacks (VM/src/lapi.cpp)
//!   - calls -> function getFirstLuauFrameDebugInfo (tests/Conformance.test.cpp)
//!   - calls -> function lua_isyieldable (VM/src/ldo.cpp)
//!   - calls -> function currentline (VM/src/ldebug.cpp)
//!   - translates_to -> rust_item conformance_tag_method_error

#[cfg(test)]
#[test]
fn conformance_tag_method_error() {
    use crate::functions::conformance_tag_method_error_setup::conformance_tag_method_error_setup;
    use crate::functions::conformance_tag_method_error_yield::conformance_tag_method_error_yield;
    use crate::functions::run_conformance::runConformance;
    use crate::records::conformance_tag_method_error_state::CONFORMANCE_TAG_METHOD_ERROR_STATE;
    use core::sync::atomic::Ordering;

    for lua_break in [false, true] {
        CONFORMANCE_TAG_METHOD_ERROR_STATE.reset(lua_break);

        runConformance(
            c"tmerror.luau".as_ptr(),
            Some(conformance_tag_method_error_setup),
            Some(conformance_tag_method_error_yield),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            false,
            core::ptr::null_mut(),
        );

        assert_eq!(
            CONFORMANCE_TAG_METHOD_ERROR_STATE
                .index
                .load(Ordering::SeqCst),
            3
        );
    }
}
