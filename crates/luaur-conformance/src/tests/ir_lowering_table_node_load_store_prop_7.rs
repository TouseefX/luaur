//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:2909:ir_lowering_table_node_load_store_prop_7`
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
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> method LoweringFixture::getCodegenAssembly (tests/IrLowering.test.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item ir_lowering_table_node_load_store_prop_7

#[cfg(test)]
#[test]
fn ir_lowering_table_node_load_store_prop_7() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(t: { x: number, y: number })
    t.x, t.y = t.y, t.x
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
  %7 = GET_SLOT_NODE_ADDR %6, 0u, K0 ('y')
  CHECK_SLOT_MATCH %7, K0 ('y'), bb_fallback_3
  %9 = LOAD_TVALUE %7, 0i
  STORE_TVALUE R1, %9
  JUMP bb_linear_11
bb_linear_11:
  %51 = GET_SLOT_NODE_ADDR %6, 2u, K1 ('x')
  CHECK_SLOT_MATCH %51, K1 ('x'), bb_fallback_5
  %53 = LOAD_TVALUE %51, 0i
  STORE_TVALUE R2, %53
  CHECK_READONLY %6, bb_fallback_7
  STORE_TVALUE %51, %9, 0i
  BARRIER_TABLE_FORWARD %6, R1, undef
  STORE_TVALUE %7, %53, 0i
  BARRIER_TABLE_FORWARD %6, R2, undef
  INTERRUPT 8u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
