//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:7514:ir_lowering_loop_step_detection_2`
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
//!   - translates_to -> rust_item ir_lowering_loop_step_detection_2

#[cfg(test)]
#[test]
fn ir_lowering_loop_step_detection_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(n: number, t: {number})
    local s = 0
    for i = 1,#t do
        s += t[i]
    end
    return s
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1) line 2
bb_0:
  CHECK_TAG R0, tnumber, exit(entry)
  CHECK_TAG R1, ttable, exit(entry)
  JUMP bb_4
bb_4:
  JUMP bb_bytecode_1
bb_bytecode_1:
  STORE_DOUBLE R2, 0
  STORE_TAG R2, tnumber
  STORE_DOUBLE R5, 1
  STORE_TAG R5, tnumber
  %12 = LOAD_POINTER R1
  CHECK_NO_METATABLE %12, bb_fallback_5
  %14 = TABLE_LEN %12
  %15 = INT_TO_NUM %14
  STORE_DOUBLE R3, %15
  STORE_TAG R3, tnumber
  JUMP bb_6
bb_6:
  STORE_DOUBLE R4, 1
  STORE_TAG R4, tnumber
  CHECK_TAG R3, tnumber, exit(4)
  CHECK_TAG R5, tnumber, exit(4)
  %28 = LOAD_DOUBLE R3
  JUMP_CMP_NUM R5, %28, not_le, bb_bytecode_3, bb_bytecode_2
bb_bytecode_2:
  INTERRUPT 5u
  CHECK_TAG R5, tnumber, exit(5)
  %36 = LOAD_POINTER R1
  %37 = LOAD_DOUBLE R5
  %38 = TRY_NUM_TO_INDEX %37, bb_fallback_7
  %39 = SUB_INT %38, 1i
  CHECK_ARRAY_SIZE %36, %39, bb_fallback_7
  CHECK_NO_METATABLE %36, bb_fallback_7
  %42 = GET_ARR_ADDR %36, %39
  %43 = LOAD_TVALUE %42
  STORE_TVALUE R6, %43
  JUMP bb_8
bb_8:
  CHECK_TAG R2, tnumber, exit(6)
  CHECK_TAG R6, tnumber, bb_fallback_9
  %53 = LOAD_DOUBLE R2
  %55 = ADD_NUM %53, R6
  STORE_DOUBLE R2, %55
  JUMP bb_10
bb_10:
  %61 = LOAD_DOUBLE R3
  %62 = LOAD_DOUBLE R5
  %63 = ADD_NUM %62, 1
  STORE_DOUBLE R5, %63
  JUMP_CMP_NUM %63, %61, le, bb_bytecode_2, bb_bytecode_3
bb_bytecode_3:
  INTERRUPT 8u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
