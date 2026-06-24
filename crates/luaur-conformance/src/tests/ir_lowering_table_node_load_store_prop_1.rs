//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:2529:ir_lowering_table_node_load_store_prop_1`
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
//!   - translates_to -> rust_item ir_lowering_table_node_load_store_prop_1

#[cfg(test)]
#[test]
fn ir_lowering_table_node_load_store_prop_1() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(t: { u: number, a: { b: number, c: { x: number, y: number } } })
    return t.a.b + t.a.c.x + t.a.c.y
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, true)
    );
    let expected = r#"
; function test($arg0) line 2
bb_0:
  CHECK_TAG R0, ttable, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %6 = LOAD_POINTER R0
  %7 = GET_SLOT_NODE_ADDR %6, 0u, K0 ('a')
  CHECK_SLOT_MATCH %7, K0 ('a'), bb_fallback_3
  %9 = LOAD_TVALUE %7, 0i
  STORE_TVALUE R3, %9
  JUMP bb_linear_23
bb_linear_23:
  CHECK_TAG R3, ttable, bb_fallback_5
  %114 = LOAD_POINTER R3
  %115 = GET_SLOT_NODE_ADDR %114, 2u, K1 ('b')
  CHECK_SLOT_MATCH %115, K1 ('b'), bb_fallback_5
  %117 = LOAD_TVALUE %115, 0i
  STORE_TVALUE R3, %117
  STORE_TVALUE R4, %9
  %129 = GET_SLOT_NODE_ADDR %114, 6u, K2 ('c')
  CHECK_SLOT_MATCH %129, K2 ('c'), bb_fallback_9
  %131 = LOAD_TVALUE %129, 0i
  STORE_TVALUE R4, %131
  CHECK_TAG R4, ttable, bb_fallback_11
  %136 = LOAD_POINTER R4
  %137 = GET_SLOT_NODE_ADDR %136, 8u, K3 ('x')
  CHECK_SLOT_MATCH %137, K3 ('x'), bb_fallback_11
  %139 = LOAD_TVALUE %137, 0i
  STORE_TVALUE R4, %139
  CHECK_TAG R3, tnumber, bb_fallback_13
  CHECK_TAG R4, tnumber, bb_fallback_13
  %146 = LOAD_DOUBLE R3
  %148 = ADD_NUM %146, R4
  STORE_DOUBLE R2, %148
  STORE_TAG R2, tnumber
  STORE_TVALUE R3, %131
  %169 = GET_SLOT_NODE_ADDR %136, 15u, K4 ('y')
  CHECK_SLOT_MATCH %169, K4 ('y'), bb_fallback_19
  %171 = LOAD_TVALUE %169, 0i
  STORE_TVALUE R3, %171
  CHECK_TAG R3, tnumber, bb_fallback_21
  %180 = ADD_NUM %148, R3
  STORE_DOUBLE R1, %180
  STORE_TAG R1, tnumber
  INTERRUPT 18u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
