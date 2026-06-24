//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3535:ir_lowering_resolve_table_path_types`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Vertex (tests/ConformanceIrHooks.h)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item ir_lowering_resolve_table_path_types

#[cfg(test)]
#[test]
fn ir_lowering_resolve_table_path_types() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
type Vertex = {pos: vector, normal: vector}

local function foo(arr: {Vertex}, i)
    local v = arr[i]

    return v.pos.Y
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 2, 2, false)
    );
    let expected = r#"
; function foo(arr, i) line 4
; R0: table [argument 'arr']
; R2: table from 0 to 6 [local 'v']
; R3: vector from 3 to 5
bb_0:
  CHECK_TAG R0, ttable, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  CHECK_TAG R1, tnumber, bb_fallback_3
  %8 = LOAD_POINTER R0
  %9 = LOAD_DOUBLE R1
  %10 = TRY_NUM_TO_INDEX %9, bb_fallback_3
  %11 = SUB_INT %10, 1i
  CHECK_ARRAY_SIZE %8, %11, bb_fallback_3
  CHECK_NO_METATABLE %8, bb_fallback_3
  %14 = GET_ARR_ADDR %8, %11
  %15 = LOAD_TVALUE %14
  STORE_TVALUE R2, %15
  JUMP bb_4
bb_4:
  CHECK_TAG R2, ttable, exit(1)
  %23 = LOAD_POINTER R2
  %24 = GET_SLOT_NODE_ADDR %23, 1u, K0 ('pos')
  CHECK_SLOT_MATCH %24, K0 ('pos'), bb_fallback_5
  %26 = LOAD_TVALUE %24, 0i
  STORE_TVALUE R3, %26
  JUMP bb_6
bb_6:
  CHECK_TAG R3, tvector, exit(3)
  %33 = LOAD_FLOAT R3, 4i
  %34 = FLOAT_TO_NUM %33
  STORE_DOUBLE R3, %34
  STORE_TAG R3, tnumber
  INTERRUPT 5u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
