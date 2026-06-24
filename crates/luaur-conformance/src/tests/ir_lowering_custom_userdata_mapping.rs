//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:4384:ir_lowering_custom_userdata_mapping`
//! Source: `tests/IrLowering.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/IrLowering.test.cpp
//! - source_includes:
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file VM/include/lualib.h
//!   - includes -> source_file Compiler/include/luacode.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file tests/ConformanceIrHooks.h
//! - incoming:
//!   - declares <- source_file tests/IrLowering.test.cpp
//! - outgoing:
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function isSupported (CodeGen/src/CodeGen.cpp)
//!   - calls -> method LoweringFixture::getCodegenAssembly (tests/IrLowering.test.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item ir_lowering_custom_userdata_mapping

#[cfg(test)]
#[test]
fn ir_lowering_custom_userdata_mapping() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
    let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);

    if luau_codegen_supported() == 0 {
        return;
    }

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: mat3)
    print(a, vec2.create(0, 0))
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0) line 2
; R0: mat3 [argument]
bb_0:
  CHECK_TAG R0, tuserdata, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  GET_CACHED_IMPORT R1, K1 (nil), 1073741824u ('print'), 1u
  %6 = LOAD_TVALUE R0, 0i, tuserdata
  STORE_TVALUE R2, %6
  GET_CACHED_IMPORT R3, K4 (nil), 2149583872u ('vec2'.'create'), 4u
  STORE_DOUBLE R4, 0
  STORE_TAG R4, tnumber
  STORE_DOUBLE R5, 0
  STORE_TAG R5, tnumber
  INTERRUPT 7u
  SET_SAVEDPC 8u
  CALL R3, 2i, -1i
  INTERRUPT 8u
  SET_SAVEDPC 9u
  CALL R1, -1i, 0i
  INTERRUPT 9u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
