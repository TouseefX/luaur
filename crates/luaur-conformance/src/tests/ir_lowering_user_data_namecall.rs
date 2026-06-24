//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1865:ir_lowering_user_data_namecall`
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
//!   - calls -> method LoweringFixture::getCodegenAssembly (tests/IrLowering.test.cpp)
//!   - translates_to -> rust_item ir_lowering_user_data_namecall

#[cfg(test)]
#[test]
fn ir_lowering_user_data_namecall() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
    let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function getxy(a: Point)
    return a:GetX() + a:GetY()
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function getxy($arg0) line 2
bb_0:
  CHECK_TAG R0, tuserdata, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  FALLBACK_NAMECALL 0u, R2, R0, K0 ('GetX')
  INTERRUPT 2u
  SET_SAVEDPC 4u
  CALL R2, 1i, 1i
  FALLBACK_NAMECALL 4u, R3, R0, K1 ('GetY')
  INTERRUPT 6u
  SET_SAVEDPC 8u
  CALL R3, 1i, 1i
  CHECK_TAG R2, tnumber, bb_fallback_3
  CHECK_TAG R3, tnumber, bb_fallback_3
  %20 = LOAD_DOUBLE R2
  %22 = ADD_NUM %20, R3
  STORE_DOUBLE R1, %22
  STORE_TAG R1, tnumber
  JUMP bb_4
bb_4:
  INTERRUPT 9u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
