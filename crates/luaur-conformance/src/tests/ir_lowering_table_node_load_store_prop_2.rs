//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:2600:ir_lowering_table_node_load_store_prop_2`
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
//!   - translates_to -> rust_item ir_lowering_table_node_load_store_prop_2

#[cfg(test)]
#[test]
fn ir_lowering_table_node_load_store_prop_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(t: { x: number, y: number }, a: number)
    t.x += a
    t.y += a * a

    t.x = t.x - t.y
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
  %9 = GET_SLOT_NODE_ADDR %8, 0u, K0 ('x')
  CHECK_SLOT_MATCH %9, K0 ('x'), bb_fallback_3
  %11 = LOAD_TVALUE %9, 0i
  STORE_TVALUE R2, %11
  JUMP bb_linear_23
bb_linear_23:
  CHECK_TAG R2, tnumber, bb_fallback_5
  %130 = LOAD_DOUBLE R2
  %131 = LOAD_DOUBLE R1
  %132 = ADD_NUM %130, %131
  STORE_DOUBLE R2, %132
  CHECK_READONLY %8, bb_fallback_7
  STORE_SPLIT_TVALUE %9, tnumber, %132, 0i
  %144 = GET_SLOT_NODE_ADDR %8, 5u, K1 ('y')
  CHECK_SLOT_MATCH %144, K1 ('y'), bb_fallback_9
  %146 = LOAD_TVALUE %144, 0i
  STORE_TVALUE R2, %146
  %150 = MUL_NUM %131, %131
  STORE_DOUBLE R3, %150
  STORE_TAG R3, tnumber
  CHECK_TAG R2, tnumber, bb_fallback_11
  %155 = LOAD_DOUBLE R2
  %156 = ADD_NUM %155, %150
  STORE_SPLIT_TVALUE %144, tnumber, %156, 0i
  %185 = SUB_NUM %132, %156
  STORE_SPLIT_TVALUE %9, tnumber, %185, 0i
  INTERRUPT 18u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
