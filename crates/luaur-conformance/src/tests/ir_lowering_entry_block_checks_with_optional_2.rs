//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1998:ir_lowering_entry_block_checks_with_optional_2`
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
//!   - translates_to -> rust_item ir_lowering_entry_block_checks_with_optional_2

#[cfg(test)]
#[test]
fn ir_lowering_entry_block_checks_with_optional_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
function eq(a: number, b: number?)
  return if b ~= nil then a + b else a
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function eq($arg0, $arg1) line 2
bb_0:
  CHECK_TAG R0, tnumber, exit(entry)
  %2 = LOAD_TAG R1
  JUMP_EQ_TAG %2, tnil, bb_3, bb_4
bb_4:
  CHECK_TAG %2, tnumber, exit(entry)
  JUMP bb_3
bb_3:
  JUMP bb_bytecode_1
bb_bytecode_1:
  JUMP_EQ_TAG R1, tnil, bb_bytecode_2, bb_5
bb_5:
  CHECK_TAG R1, tnumber, exit(2)
  %13 = LOAD_DOUBLE R0
  %15 = ADD_NUM %13, R1
  STORE_DOUBLE R2, %15
  STORE_TAG R2, tnumber
  INTERRUPT 3u
  RETURN R2, 1i
bb_bytecode_2:
  %20 = LOAD_TVALUE R0, 0i, tnumber
  STORE_TVALUE R2, %20
  INTERRUPT 5u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
