//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1219:ir_lowering_vector_random_prop`
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
//!   - translates_to -> rust_item ir_lowering_vector_random_prop

#[cfg(test)]
#[test]
fn ir_lowering_vector_random_prop() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: vector)
    return a.XX + a.YY + a.ZZ
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0) line 2
bb_0:
  CHECK_TAG R0, tvector, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  FALLBACK_GETTABLEKS 0u, R3, R0, K0 ('XX')
  FALLBACK_GETTABLEKS 2u, R4, R0, K1 ('YY')
  CHECK_TAG R3, tnumber, bb_fallback_3
  CHECK_TAG R4, tnumber, bb_fallback_3
  %14 = LOAD_DOUBLE R3
  %16 = ADD_NUM %14, R4
  STORE_DOUBLE R2, %16
  STORE_TAG R2, tnumber
  JUMP bb_4
bb_4:
  FALLBACK_GETTABLEKS 5u, R3, R0, K2 ('ZZ')
  CHECK_TAG R2, tnumber, bb_fallback_5
  CHECK_TAG R3, tnumber, bb_fallback_5
  %30 = LOAD_DOUBLE R2
  %32 = ADD_NUM %30, R3
  STORE_DOUBLE R1, %32
  STORE_TAG R1, tnumber
  JUMP bb_6
bb_6:
  INTERRUPT 8u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
