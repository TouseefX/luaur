//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:2763:ir_lowering_table_node_load_store_prop_5`
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
//!   - translates_to -> rust_item ir_lowering_table_node_load_store_prop_5

#[cfg(test)]
#[test]
fn ir_lowering_table_node_load_store_prop_5() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _linear_setup_entry_state =
        ScopedFastFlag::new(&FFlag::LuauCodegenLinearSetupEntryState3, true);
    let _record_all_block_exit_info =
        ScopedFastFlag::new(&FFlag::LuauCodegenRecordAllBlockExitInfo, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(t: { w: number, h: number, data: {vector} }, uv: vector)
    uv *= vector.create(t.w, t.h)
    uv -= vector.create(.5,.5)
    local uv0 = vector.floor(uv)
    local uv1 = vector.ceil(uv)
    local a = uv - uv0
    local x0 = uv0.x % t.w
    local x1 = uv1.x % t.w
    local y0 = (uv0.y % t.h) * t.w
    local y1 = (uv1.y % t.h) * t.w
    return a, x0, x1, y0, y1
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
  CHECK_TAG R1, tvector, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %8 = LOAD_POINTER R0
  %9 = GET_SLOT_NODE_ADDR %8, 0u, K0 ('w')
  CHECK_SLOT_MATCH %9, K0 ('w'), bb_fallback_3
  %11 = LOAD_TVALUE %9, 0i
  STORE_TVALUE R3, %11
  JUMP bb_linear_34
bb_linear_34:
  %248 = GET_SLOT_NODE_ADDR %8, 2u, K1 ('h')
  CHECK_SLOT_MATCH %248, K1 ('h'), bb_fallback_5
  %250 = LOAD_TVALUE %248, 0i
  STORE_TVALUE R4, %250
  CHECK_SAFE_ENV exit(4)
  CHECK_TAG R3, tnumber, exit(6)
  CHECK_TAG R4, tnumber, exit(6)
  %258 = LOAD_DOUBLE R3
  %259 = LOAD_DOUBLE R4
  %260 = NUM_TO_FLOAT %258
  %261 = NUM_TO_FLOAT %259
  STORE_VECTOR R2, %260, %261, 0
  STORE_TAG R2, tvector
  %266 = LOAD_TVALUE R1, 0i, tvector
  %267 = LOAD_TVALUE R2, 0i, tvector
  %268 = MUL_VEC %266, %267
  %271 = LOAD_TVALUE K5 (0.5, 0.5, 0), 0i, tvector
  %273 = SUB_VEC %268, %271
  %276 = FLOOR_VEC %273
  %279 = CEIL_VEC %273
  %282 = SUB_VEC %273, %276
  %283 = TAG_VECTOR %282
  STORE_TVALUE R4, %283
  %285 = EXTRACT_VEC %276, 0i
  %286 = FLOAT_TO_NUM %285
  STORE_TVALUE R7, %11
  %301 = MOD_NUM %286, %258
  STORE_DOUBLE R5, %301
  STORE_TAG R5, tnumber
  %307 = EXTRACT_VEC %279, 0i
  %308 = FLOAT_TO_NUM %307
  STORE_DOUBLE R7, %308
  STORE_TVALUE R8, %11
  %323 = MOD_NUM %308, %258
  STORE_SPLIT_TVALUE R6, tnumber, %323
  %329 = EXTRACT_VEC %276, 1i
  %330 = FLOAT_TO_NUM %329
  STORE_TVALUE R10, %250
  %345 = MOD_NUM %330, %259
  STORE_DOUBLE R8, %345
  STORE_TVALUE R9, %11
  %361 = MUL_NUM %345, %258
  STORE_DOUBLE R7, %361
  %367 = EXTRACT_VEC %279, 1i
  %368 = FLOAT_TO_NUM %367
  STORE_DOUBLE R10, %368
  %383 = MOD_NUM %368, %259
  STORE_DOUBLE R9, %383
  %399 = MUL_NUM %383, %258
  STORE_DOUBLE R8, %399
  INTERRUPT 49u
  RETURN R4, 5i
"#;

    assert_eq!(actual, expected);
}
