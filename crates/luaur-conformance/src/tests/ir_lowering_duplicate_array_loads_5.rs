//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:2426:ir_lowering_duplicate_array_loads_5`
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
//!   - translates_to -> rust_item ir_lowering_duplicate_array_loads_5

#[cfg(test)]
#[test]
fn ir_lowering_duplicate_array_loads_5() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(t: { x: number, y: number })
    t[1] = 14
    t[2] = 28

    t[1] = t[1] - t[2]
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
  STORE_DOUBLE R1, 14
  STORE_TAG R1, tnumber
  %8 = LOAD_POINTER R0
  CHECK_ARRAY_SIZE %8, 0i, bb_fallback_3
  CHECK_NO_METATABLE %8, bb_fallback_3
  CHECK_READONLY %8, bb_fallback_3
  %12 = GET_ARR_ADDR %8, 0i
  STORE_SPLIT_TVALUE %12, tnumber, 14, 0i
  JUMP bb_linear_15
bb_linear_15:
  STORE_DOUBLE R1, 28
  CHECK_ARRAY_SIZE %8, 1i, bb_fallback_5
  STORE_SPLIT_TVALUE %12, tnumber, 28, 16i
  STORE_SPLIT_TVALUE %12, tnumber, -14, 0i
  INTERRUPT 8u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
