//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:1229:conformance_integers`
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
//!   - calls -> function setupNativeHelpers (tests/Conformance.test.cpp)
//!   - calls -> function luau_codegen_supported (CodeGen/src/lcodegen.cpp)
//!   - translates_to -> rust_item conformance_integers

#[cfg(test)]
#[test]
fn conformance_integers() {
    use crate::functions::run_conformance::{runConformance, CODEGEN};
    use crate::functions::setup_native_helpers::setup_native_helpers;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use luaur_common::FFlag;

    let _ncg_buffer_integer = ScopedFastFlag::new(&FFlag::LuauCodegenBufferInteger, true);
    let _luau_codegen_fix_buffer_len_check =
        ScopedFastFlag::new(&FFlag::LuauCodegenFixBufferLenCheck, true);

    if FFlag::LuauIntegerType2.get() && FFlag::LuauIntegerLibrary.get() {
        runConformance(
            c"integers.luau".as_ptr(),
            Some(setup_native_helpers),
            None,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            false,
            core::ptr::null_mut(),
        );

        if unsafe { CODEGEN } && luau_codegen_supported() != 0 {
            runConformance(
                c"integers_regspill.luau".as_ptr(),
                Some(setup_native_helpers),
                None,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                false,
                core::ptr::null_mut(),
            );
        }
    }
}
