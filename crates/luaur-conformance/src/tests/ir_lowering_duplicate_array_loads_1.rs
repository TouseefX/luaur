//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:2143:ir_lowering_duplicate_array_loads_1`
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
//!   - translates_to -> rust_item ir_lowering_duplicate_array_loads_1

#[cfg(test)]
#[test]
fn ir_lowering_duplicate_array_loads_1() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(n: number, t: {number}, u: {number})
    return t[n] * t[n] + u[n] * u[n]
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
  CHECK_TAG R0, tnumber, exit(entry)
  CHECK_TAG R1, ttable, exit(entry)
  CHECK_TAG R2, ttable, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %12 = LOAD_POINTER R1
  %13 = LOAD_DOUBLE R0
  %14 = TRY_NUM_TO_INDEX %13, bb_fallback_3
  %15 = SUB_INT %14, 1i
  CHECK_ARRAY_SIZE %12, %15, bb_fallback_3
  CHECK_NO_METATABLE %12, bb_fallback_3
  %18 = GET_ARR_ADDR %12, %15
  %19 = LOAD_TVALUE %18
  STORE_TVALUE R5, %19
  JUMP bb_linear_17
bb_linear_17:
  STORE_TVALUE R6, %19
  CHECK_TAG R5, tnumber, bb_fallback_7
  %131 = LOAD_DOUBLE R5
  %133 = MUL_NUM %131, %131
  STORE_DOUBLE R4, %133
  STORE_TAG R4, tnumber
  %137 = LOAD_POINTER R2
  CHECK_ARRAY_SIZE %137, %15, bb_fallback_9
  CHECK_NO_METATABLE %137, bb_fallback_9
  %143 = GET_ARR_ADDR %137, %15
  %144 = LOAD_TVALUE %143
  STORE_TVALUE R6, %144
  STORE_TVALUE R7, %144
  CHECK_TAG R6, tnumber, bb_fallback_13
  %161 = LOAD_DOUBLE R6
  %163 = MUL_NUM %161, %161
  %173 = ADD_NUM %133, %163
  STORE_DOUBLE R3, %173
  STORE_TAG R3, tnumber
  INTERRUPT 7u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
