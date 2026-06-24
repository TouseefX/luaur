//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:787:ir_lowering_boolean_compare`
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
//!   - translates_to -> rust_item ir_lowering_boolean_compare

#[cfg(test)]
#[test]
fn ir_lowering_boolean_compare() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a)
    return { a == true, a == false, a ~= true, a ~= false }
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0) line 2
bb_bytecode_0:
  SET_SAVEDPC 1u
  %1 = NEW_TABLE 4u, 0u
  STORE_POINTER R1, %1
  STORE_TAG R1, ttable
  CHECK_GC
  %5 = LOAD_TAG R0
  %6 = LOAD_INT R0
  %7 = CMP_SPLIT_TVALUE %5, tboolean, %6, 1i, eq
  STORE_TAG R2, tboolean
  STORE_INT R2, %7
  JUMP bb_bytecode_2
bb_bytecode_2:
  %16 = CMP_SPLIT_TVALUE %5, tboolean, %6, 0i, eq
  STORE_TAG R3, tboolean
  STORE_INT R3, %16
  JUMP bb_bytecode_4
bb_bytecode_4:
  %25 = CMP_SPLIT_TVALUE %5, tboolean, %6, 1i, not_eq
  STORE_TAG R4, tboolean
  STORE_INT R4, %25
  JUMP bb_bytecode_6
bb_bytecode_6:
  %34 = CMP_SPLIT_TVALUE %5, tboolean, %6, 0i, not_eq
  STORE_TAG R5, tboolean
  STORE_INT R5, %34
  JUMP bb_bytecode_8
bb_bytecode_8:
  SETLIST 18u, R1, R2, 4i, 1u, 4u
  INTERRUPT 20u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
