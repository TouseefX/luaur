//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1906:ir_lowering_entry_block_checks_are_not_inferred`
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
//!   - translates_to -> rust_item ir_lowering_entry_block_checks_are_not_inferred

#[cfg(test)]
#[test]
fn ir_lowering_entry_block_checks_are_not_inferred() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
function eq(a: number, b: number, limit)
  if not limit then limit = 0.125 end
  return math.abs(a - b) <= limit
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function eq($arg0, $arg1, $arg2) line 2
bb_0:
  CHECK_TAG R0, tnumber, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  JUMP bb_5
bb_5:
  JUMP bb_bytecode_1
bb_bytecode_1:
  JUMP_IF_TRUTHY R2, bb_bytecode_2, bb_6
bb_6:
  STORE_DOUBLE R2, 0.125
  STORE_TAG R2, tnumber
  JUMP bb_bytecode_2
bb_bytecode_2:
  implicit CHECK_SAFE_ENV exit(2)
  %14 = LOAD_DOUBLE R0
  %16 = SUB_NUM %14, R1
  %23 = ABS_NUM %16
  STORE_DOUBLE R4, %23
  STORE_TAG R4, tnumber
  CHECK_TAG R2, tnumber, bb_fallback_9
  %32 = LOAD_DOUBLE R2
  JUMP_CMP_NUM %23, %32, le, bb_bytecode_3, bb_8
bb_8:
  STORE_INT R3, 0i
  STORE_TAG R3, tboolean
  JUMP bb_bytecode_4
bb_bytecode_3:
  STORE_INT R3, 1i
  STORE_TAG R3, tboolean
  JUMP bb_bytecode_4
bb_bytecode_4:
  INTERRUPT 11u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
