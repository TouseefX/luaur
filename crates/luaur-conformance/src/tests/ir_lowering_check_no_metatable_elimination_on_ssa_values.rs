//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3055:ir_lowering_check_no_metatable_elimination_on_ssa_values`
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
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item ir_lowering_check_no_metatable_elimination_on_ssa_values

#[cfg(test)]
#[test]
fn ir_lowering_check_no_metatable_elimination_on_ssa_values() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _extra_table_opts = ScopedFastFlag::new(&FFlag::LuauCodegenExtraTableOpts, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(t: { y: { z: number } })
    t.y[1] = t.y.z
    t.y[2] = 20
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
  %84 = LOAD_POINTER R2
  %85 = GET_SLOT_NODE_ADDR %84, 4u, K1 ('z')
  CHECK_SLOT_MATCH %85, K1 ('z'), bb_fallback_7
  %87 = LOAD_TVALUE %85, 0i
  STORE_TVALUE R2, %87
  CHECK_ARRAY_SIZE %84, 0i, bb_fallback_9
  CHECK_NO_METATABLE %84, bb_fallback_9
  CHECK_READONLY %84, bb_fallback_9
  %96 = GET_ARR_ADDR %84, 0i
  STORE_TVALUE %96, %87, 0i
  BARRIER_TABLE_FORWARD %84, R2, undef
  STORE_DOUBLE R2, 20
  STORE_TAG R2, tnumber
  CHECK_ARRAY_SIZE %84, 1i, bb_fallback_13
  STORE_SPLIT_TVALUE %96, tnumber, 20, 16i
  INTERRUPT 11u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
