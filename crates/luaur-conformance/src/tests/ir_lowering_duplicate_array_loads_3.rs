//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:2286:ir_lowering_duplicate_array_loads_3`
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
//!   - translates_to -> rust_item ir_lowering_duplicate_array_loads_3

#[cfg(test)]
#[test]
fn ir_lowering_duplicate_array_loads_3() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(t: { x: number, y: number }, a: number)
    t[1] += a
    t[2] += a * a

    t[1] = t[1] - t[2]
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, true)
    );
    let expected = r#"
; function test($arg0, $arg1) line 2
bb_0:
  CHECK_TAG R0, ttable, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %8 = LOAD_POINTER R0
  CHECK_ARRAY_SIZE %8, 0i, bb_fallback_3
  CHECK_NO_METATABLE %8, bb_fallback_3
  %11 = GET_ARR_ADDR %8, 0i
  %12 = LOAD_TVALUE %11, 0i
  STORE_TVALUE R2, %12
  JUMP bb_linear_23
bb_linear_23:
  CHECK_TAG R2, tnumber, bb_fallback_5
  %144 = LOAD_DOUBLE R2
  %145 = LOAD_DOUBLE R1
  %146 = ADD_NUM %144, %145
  STORE_DOUBLE R2, %146
  CHECK_READONLY %8, bb_fallback_7
  STORE_SPLIT_TVALUE %11, tnumber, %146, 0i
  CHECK_ARRAY_SIZE %8, 1i, bb_fallback_9
  %162 = LOAD_TVALUE %11, 16i
  STORE_TVALUE R2, %162
  %166 = MUL_NUM %145, %145
  STORE_DOUBLE R3, %166
  STORE_TAG R3, tnumber
  CHECK_TAG R2, tnumber, bb_fallback_11
  %171 = LOAD_DOUBLE R2
  %172 = ADD_NUM %171, %166
  STORE_SPLIT_TVALUE %11, tnumber, %172, 16i
  %204 = SUB_NUM %146, %172
  STORE_SPLIT_TVALUE %11, tnumber, %204, 0i
  INTERRUPT 11u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
