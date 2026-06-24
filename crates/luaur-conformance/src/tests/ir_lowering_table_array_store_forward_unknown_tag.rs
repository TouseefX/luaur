//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3213:ir_lowering_table_array_store_forward_unknown_tag`
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
//!   - translates_to -> rust_item ir_lowering_table_array_store_forward_unknown_tag

#[cfg(test)]
#[test]
fn ir_lowering_table_array_store_forward_unknown_tag() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _extra_table_opts = ScopedFastFlag::new(&FFlag::LuauCodegenExtraTableOpts, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(t: {}, v, w)
    t[1] = v
    t[2] = w
    return t[1]
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
  CHECK_ARRAY_SIZE %6, 0i, bb_fallback_3
  CHECK_NO_METATABLE %6, bb_fallback_3
  CHECK_READONLY %6, bb_fallback_3
  %10 = GET_ARR_ADDR %6, 0i
  %11 = LOAD_TVALUE R1
  STORE_TVALUE %10, %11, 0i
  BARRIER_TABLE_FORWARD %6, R1, undef
  JUMP bb_linear_9
bb_linear_9:
  CHECK_ARRAY_SIZE %6, 1i, bb_fallback_5
  %51 = LOAD_TVALUE R2
  STORE_TVALUE %10, %51, 16i
  BARRIER_TABLE_FORWARD %6, R2, undef
  STORE_TVALUE R3, %11
  INTERRUPT 3u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
