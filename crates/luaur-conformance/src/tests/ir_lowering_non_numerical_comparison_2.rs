//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:5110:ir_lowering_non_numerical_comparison_2`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item ir_lowering_non_numerical_comparison_2

#[cfg(test)]
#[test]
fn ir_lowering_non_numerical_comparison_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: string, b: string, c: {}, d: {})
    return a > b and c > d
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1, $arg2, $arg3) line 2
bb_0:
  CHECK_TAG R0, tstring, exit(entry)
  CHECK_TAG R1, tstring, exit(entry)
  CHECK_TAG R2, ttable, exit(entry)
  CHECK_TAG R3, ttable, exit(entry)
  JUMP bb_4
bb_4:
  JUMP bb_bytecode_1
bb_bytecode_1:
  STORE_INT R4, 0i
  STORE_TAG R4, tboolean
  SET_SAVEDPC 2u
  %13 = CMP_ANY R1, R0, lt
  JUMP_CMP_INT %13, 0i, eq, bb_bytecode_3, bb_5
bb_5:
  SET_SAVEDPC 4u
  %16 = CMP_ANY R3, R2, lt
  JUMP_CMP_INT %16, 0i, eq, bb_6, bb_bytecode_2
bb_6:
  STORE_INT R4, 0i
  STORE_TAG R4, tboolean
  JUMP bb_bytecode_3
bb_bytecode_2:
  STORE_INT R4, 1i
  STORE_TAG R4, tboolean
  JUMP bb_bytecode_3
bb_bytecode_3:
  INTERRUPT 7u
  RETURN R4, 1i
"#;

    assert_eq!(actual, expected);
}
