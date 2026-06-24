//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:2348:ir_lowering_duplicate_array_loads_4`
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
//!   - translates_to -> rust_item ir_lowering_duplicate_array_loads_4

#[cfg(test)]
#[test]
fn ir_lowering_duplicate_array_loads_4() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(t: { x: number, y: number }, a: number, i: number)
    t[i] += a
    t[i + 1] += a * a

    t[i] = t[i] - t[i + 1]
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
  %12 = LOAD_POINTER R0
  %13 = LOAD_DOUBLE R2
  %14 = TRY_NUM_TO_INDEX %13, bb_fallback_3
  %15 = SUB_INT %14, 1i
  CHECK_ARRAY_SIZE %12, %15, bb_fallback_3
  CHECK_NO_METATABLE %12, bb_fallback_3
  %18 = GET_ARR_ADDR %12, %15
  %19 = LOAD_TVALUE %18
  STORE_TVALUE R3, %19
  JUMP bb_linear_23
bb_linear_23:
  CHECK_TAG R3, tnumber, bb_fallback_5
  %193 = LOAD_DOUBLE R3
  %194 = LOAD_DOUBLE R1
  %195 = ADD_NUM %193, %194
  STORE_DOUBLE R3, %195
  CHECK_READONLY %12, bb_fallback_7
  STORE_SPLIT_TVALUE %18, tnumber, %195
  %211 = ADD_NUM %13, 1
  STORE_DOUBLE R3, %211
  %215 = TRY_NUM_TO_INDEX %211, bb_fallback_9
  %216 = SUB_INT %215, 1i
  CHECK_ARRAY_SIZE %12, %216, bb_fallback_9
  %219 = GET_ARR_ADDR %12, %216
  %220 = LOAD_TVALUE %219
  STORE_TVALUE R4, %220
  %224 = MUL_NUM %194, %194
  STORE_DOUBLE R5, %224
  STORE_TAG R5, tnumber
  CHECK_TAG R4, tnumber, bb_fallback_11
  %229 = LOAD_DOUBLE R4
  %230 = ADD_NUM %229, %224
  STORE_SPLIT_TVALUE %219, tnumber, %230
  %254 = LOAD_TVALUE %18
  STORE_TVALUE R4, %254
  %267 = LOAD_TVALUE %219
  STORE_TVALUE R5, %267
  CHECK_TAG R4, tnumber, bb_fallback_19
  %274 = LOAD_DOUBLE R4
  %276 = SUB_NUM %274, %230
  STORE_SPLIT_TVALUE %18, tnumber, %276
  INTERRUPT 13u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
