//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3165:ir_lowering_table_store_forward_unknown_tag`
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
//!   - translates_to -> rust_item ir_lowering_table_store_forward_unknown_tag

#[cfg(test)]
#[test]
fn ir_lowering_table_store_forward_unknown_tag() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _extra_table_opts = ScopedFastFlag::new(&FFlag::LuauCodegenExtraTableOpts, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(t: {}, v, w)
    t.x = v
    t.y = w
    return t.x
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, true)
    );
    let expected = r#"
; function foo($arg0, $arg1, $arg2) line 2
bb_0:
  CHECK_TAG R0, ttable, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %6 = LOAD_POINTER R0
  %7 = GET_SLOT_NODE_ADDR %6, 0u, K0 ('x')
  CHECK_SLOT_MATCH %7, K0 ('x'), bb_fallback_3
  CHECK_READONLY %6, bb_fallback_3
  %10 = LOAD_TVALUE R1
  STORE_TVALUE %7, %10, 0i
  BARRIER_TABLE_FORWARD %6, R1, undef
  JUMP bb_linear_9
bb_linear_9:
  %41 = GET_SLOT_NODE_ADDR %6, 2u, K1 ('y')
  CHECK_SLOT_MATCH %41, K1 ('y'), bb_fallback_5
  %44 = LOAD_TVALUE R2
  STORE_TVALUE %41, %44, 0i
  BARRIER_TABLE_FORWARD %6, R2, undef
  CHECK_NODE_VALUE %7, bb_fallback_7
  STORE_TVALUE R3, %10
  INTERRUPT 6u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
