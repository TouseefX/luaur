//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:914:ir_lowering_number_compare_3`
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
//!   - translates_to -> rust_item ir_lowering_number_compare_3

#[cfg(test)]
#[test]
fn ir_lowering_number_compare_3() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: number, b: number, c: number)
    return { a == b, a ~= c }
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1, $arg2) line 2
bb_0:
  CHECK_TAG R0, tnumber, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  CHECK_TAG R2, tnumber, exit(entry)
  JUMP bb_6
bb_6:
  JUMP bb_bytecode_1
bb_bytecode_1:
  SET_SAVEDPC 1u
  %9 = NEW_TABLE 2u, 0u
  STORE_POINTER R3, %9
  STORE_TAG R3, ttable
  CHECK_GC
  %17 = LOAD_DOUBLE R0
  %18 = LOAD_DOUBLE R1
  %19 = CMP_SPLIT_TVALUE tnumber, tnumber, %17, %18, eq
  STORE_INT R4, %19
  STORE_TAG R4, tboolean
  JUMP bb_bytecode_3
bb_bytecode_3:
  %31 = LOAD_DOUBLE R2
  %32 = CMP_SPLIT_TVALUE tnumber, tnumber, %17, %31, not_eq
  STORE_INT R5, %32
  STORE_TAG R5, tboolean
  JUMP bb_bytecode_5
bb_bytecode_5:
  SETLIST 10u, R3, R4, 2i, 1u, 2u
  INTERRUPT 12u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
