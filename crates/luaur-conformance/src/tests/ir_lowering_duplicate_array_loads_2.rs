//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:2205:ir_lowering_duplicate_array_loads_2`
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
//!   - translates_to -> rust_item ir_lowering_duplicate_array_loads_2

#[cfg(test)]
#[test]
fn ir_lowering_duplicate_array_loads_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _extra_table_opts = ScopedFastFlag::new(&FFlag::LuauCodegenExtraTableOpts, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(t, a: number, b: number)
    return t[a][b].x + t[a][b].y + t[a][b].z
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
  CHECK_TAG R1, tnumber, exit(entry)
  CHECK_TAG R2, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  CHECK_TAG R0, ttable, bb_fallback_3
  %10 = LOAD_POINTER R0
  %11 = LOAD_DOUBLE R1
  %12 = TRY_NUM_TO_INDEX %11, bb_fallback_3
  %13 = SUB_INT %12, 1i
  CHECK_ARRAY_SIZE %10, %13, bb_fallback_3
  CHECK_NO_METATABLE %10, bb_fallback_3
  %16 = GET_ARR_ADDR %10, %13
  %17 = LOAD_TVALUE %16
  STORE_TVALUE R6, %17
  JUMP bb_linear_25
bb_linear_25:
  CHECK_TAG R6, ttable, bb_fallback_5
  %168 = LOAD_POINTER R6
  %169 = LOAD_DOUBLE R2
  %170 = TRY_NUM_TO_INDEX %169, bb_fallback_5
  %171 = SUB_INT %170, 1i
  CHECK_ARRAY_SIZE %168, %171, bb_fallback_5
  CHECK_NO_METATABLE %168, bb_fallback_5
  %174 = GET_ARR_ADDR %168, %171
  %175 = LOAD_TVALUE %174
  STORE_TVALUE R5, %175
  CHECK_TAG R5, ttable, bb_fallback_7
  %180 = LOAD_POINTER R5
  %181 = GET_SLOT_NODE_ADDR %180, 2u, K0 ('x')
  CHECK_SLOT_MATCH %181, K0 ('x'), bb_fallback_7
  %183 = LOAD_TVALUE %181, 0i
  STORE_TVALUE R5, %183
  STORE_TVALUE R6, %175
  %213 = GET_SLOT_NODE_ADDR %180, 6u, K1 ('y')
  CHECK_SLOT_MATCH %213, K1 ('y'), bb_fallback_13
  %215 = LOAD_TVALUE %213, 0i
  STORE_TVALUE R6, %215
  CHECK_TAG R5, tnumber, bb_fallback_15
  CHECK_TAG R6, tnumber, bb_fallback_15
  %222 = LOAD_DOUBLE R5
  %224 = ADD_NUM %222, R6
  STORE_DOUBLE R4, %224
  STORE_TAG R4, tnumber
  STORE_TVALUE R5, %175
  %255 = GET_SLOT_NODE_ADDR %180, 11u, K2 ('z')
  CHECK_SLOT_MATCH %255, K2 ('z'), bb_fallback_21
  %257 = LOAD_TVALUE %255, 0i
  STORE_TVALUE R5, %257
  CHECK_TAG R5, tnumber, bb_fallback_23
  %266 = ADD_NUM %224, R5
  STORE_DOUBLE R3, %266
  STORE_TAG R3, tnumber
  INTERRUPT 14u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
