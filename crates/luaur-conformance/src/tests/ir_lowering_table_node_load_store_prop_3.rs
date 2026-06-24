//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:2660:ir_lowering_table_node_load_store_prop_3`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item ir_lowering_table_node_load_store_prop_3

#[cfg(test)]
#[test]
fn ir_lowering_table_node_load_store_prop_3() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(t: { x: number, y: number }, a: string)
    t.x = 2
    t[a] = 4
    return t.x * 2
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
  CHECK_TAG R1, tstring, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  STORE_DOUBLE R2, 2
  STORE_TAG R2, tnumber
  %10 = LOAD_POINTER R0
  %11 = GET_SLOT_NODE_ADDR %10, 1u, K0 ('x')
  CHECK_SLOT_MATCH %11, K0 ('x'), bb_fallback_3
  CHECK_READONLY %10, bb_fallback_3
  STORE_SPLIT_TVALUE %11, tnumber, 2, 0i
  JUMP bb_linear_9
bb_linear_9:
  STORE_DOUBLE R2, 4
  SET_SAVEDPC 5u
  SET_TABLE R2, R0, R1
  %50 = LOAD_POINTER R0
  %51 = GET_SLOT_NODE_ADDR %50, 5u, K0 ('x')
  CHECK_SLOT_MATCH %51, K0 ('x'), bb_fallback_5
  %53 = LOAD_TVALUE %51, 0i
  STORE_TVALUE R3, %53
  CHECK_TAG R3, tnumber, bb_fallback_7
  %58 = LOAD_DOUBLE R3
  %59 = ADD_NUM %58, %58
  STORE_DOUBLE R2, %59
  INTERRUPT 8u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
