//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:871:ir_lowering_number_compare_2`
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
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item ir_lowering_number_compare_2

#[cfg(test)]
#[test]
fn ir_lowering_number_compare_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a, b, c)
    return { a == b, a ~= c }
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1, $arg2) line 2
bb_bytecode_0:
  SET_SAVEDPC 1u
  %1 = NEW_TABLE 2u, 0u
  STORE_POINTER R3, %1
  STORE_TAG R3, ttable
  CHECK_GC
  CHECK_TAG R0, tnumber, bb_fallback_5
  CHECK_TAG R1, tnumber, bb_fallback_5
  %9 = LOAD_DOUBLE R0
  %10 = LOAD_DOUBLE R1
  %11 = CMP_SPLIT_TVALUE tnumber, tnumber, %9, %10, eq
  STORE_INT R4, %11
  STORE_TAG R4, tboolean
  JUMP bb_bytecode_2
bb_bytecode_2:
  CHECK_TAG R0, tnumber, bb_fallback_6
  CHECK_TAG R2, tnumber, bb_fallback_6
  %27 = LOAD_DOUBLE R0
  %28 = LOAD_DOUBLE R2
  %29 = CMP_SPLIT_TVALUE tnumber, tnumber, %27, %28, not_eq
  STORE_INT R5, %29
  STORE_TAG R5, tboolean
  JUMP bb_bytecode_4
bb_bytecode_4:
  SETLIST 10u, R3, R4, 2i, 1u, undef
  INTERRUPT 12u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
