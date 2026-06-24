//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3000:ir_lowering_check_readonly_elimination_on_ssa_values`
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
//!   - type_ref -> record RegisterLink (CodeGen/src/OptimizeConstProp.cpp)
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item ir_lowering_check_readonly_elimination_on_ssa_values

#[cfg(test)]
#[test]
fn ir_lowering_check_readonly_elimination_on_ssa_values() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _extra_table_opts = ScopedFastFlag::new(&FFlag::LuauCodegenExtraTableOpts, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(t: { y: { a: number, b: number, c: number } })
    t.y.a = t.y.b -- this kills 'readonly' state tracking through VM RegisterLink
    t.y.c = 3
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, true)
    );
    let expected = r#"
; function foo($arg0) line 2
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
  JUMP bb_linear_15
bb_linear_15:
  STORE_TVALUE R2, %9
  CHECK_TAG R2, ttable, bb_fallback_7
  %80 = LOAD_POINTER R2
  %81 = GET_SLOT_NODE_ADDR %80, 4u, K1 ('b')
  CHECK_SLOT_MATCH %81, K1 ('b'), bb_fallback_7
  %83 = LOAD_TVALUE %81, 0i
  STORE_TVALUE R2, %83
  %89 = GET_SLOT_NODE_ADDR %80, 6u, K2 ('a')
  CHECK_SLOT_MATCH %89, K2 ('a'), bb_fallback_9
  CHECK_READONLY %80, bb_fallback_9
  STORE_TVALUE %89, %83, 0i
  BARRIER_TABLE_FORWARD %80, R2, undef
  STORE_DOUBLE R2, 3
  STORE_TAG R2, tnumber
  %107 = GET_SLOT_NODE_ADDR %80, 11u, K3 ('c')
  CHECK_SLOT_MATCH %107, K3 ('c'), bb_fallback_13
  STORE_SPLIT_TVALUE %107, tnumber, 3, 0i
  INTERRUPT 13u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
