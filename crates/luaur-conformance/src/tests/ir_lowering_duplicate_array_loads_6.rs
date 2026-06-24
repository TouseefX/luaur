//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:2472:ir_lowering_duplicate_array_loads_6`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - translates_to -> rust_item ir_lowering_duplicate_array_loads_6

#[cfg(test)]
#[test]
fn ir_lowering_duplicate_array_loads_6() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(t: { x: number, y: number }, a: number, i: number)
    t[i] = 2
    t[2] = 4
    return t[i] * 2
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, true)
    );
    let expected = r#"
; function test($arg0, $arg1, $arg2) line 2
bb_0:
  CHECK_TAG R0, ttable, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  CHECK_TAG R2, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  STORE_DOUBLE R3, 2
  STORE_TAG R3, tnumber
  %14 = LOAD_POINTER R0
  %15 = LOAD_DOUBLE R2
  %16 = TRY_NUM_TO_INDEX %15, bb_fallback_3
  %17 = SUB_INT %16, 1i
  CHECK_ARRAY_SIZE %14, %17, bb_fallback_3
  CHECK_NO_METATABLE %14, bb_fallback_3
  CHECK_READONLY %14, bb_fallback_3
  %21 = GET_ARR_ADDR %14, %17
  STORE_SPLIT_TVALUE %21, tnumber, 2
  JUMP bb_linear_11
bb_linear_11:
  STORE_DOUBLE R3, 4
  CHECK_ARRAY_SIZE %14, 1i, bb_fallback_5
  %80 = GET_ARR_ADDR %14, 0i
  STORE_SPLIT_TVALUE %80, tnumber, 4, 16i
  %90 = LOAD_TVALUE %21
  STORE_TVALUE R4, %90
  CHECK_TAG R4, tnumber, bb_fallback_9
  %95 = LOAD_DOUBLE R4
  %96 = ADD_NUM %95, %95
  STORE_DOUBLE R3, %96
  INTERRUPT 6u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
