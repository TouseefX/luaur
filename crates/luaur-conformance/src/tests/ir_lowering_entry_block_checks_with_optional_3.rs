//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:2039:ir_lowering_entry_block_checks_with_optional_3`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item ir_lowering_entry_block_checks_with_optional_3

#[cfg(test)]
#[test]
fn ir_lowering_entry_block_checks_with_optional_3() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
function eq(a: string, b: string?, c: {string}?, d: number?, e: {x: number}, f: number?)
  if b then
    return a
  else
    return c
  end
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function eq($arg0, $arg1, $arg2, $arg3, $arg4, $arg5) line 2
bb_0:
  CHECK_TAG R0, tstring, exit(entry)
  %2 = LOAD_TAG R1
  JUMP_EQ_TAG %2, tnil, bb_3, bb_4
bb_4:
  CHECK_TAG %2, tstring, exit(entry)
  JUMP bb_3
bb_3:
  %6 = LOAD_TAG R2
  JUMP_EQ_TAG %6, tnil, bb_5, bb_6
bb_6:
  CHECK_TAG %6, ttable, exit(entry)
  JUMP bb_5
bb_5:
  %10 = LOAD_TAG R3
  JUMP_EQ_TAG %10, tnil, bb_7, bb_8
bb_8:
  CHECK_TAG %10, tnumber, exit(entry)
  JUMP bb_7
bb_7:
  CHECK_TAG R4, ttable, exit(entry)
  %16 = LOAD_TAG R5
  JUMP_EQ_TAG %16, tnil, bb_9, bb_10
bb_10:
  CHECK_TAG %16, tnumber, exit(entry)
  JUMP bb_9
bb_9:
  JUMP bb_bytecode_1
bb_bytecode_1:
  JUMP_IF_FALSY R1, bb_bytecode_2, bb_11
bb_11:
  INTERRUPT 1u
  RETURN R0, 1i
bb_bytecode_2:
  INTERRUPT 2u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
