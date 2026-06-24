//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1727:ir_lowering_vector_number_mixed_2`
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
//!   - translates_to -> rust_item ir_lowering_vector_number_mixed_2

#[cfg(test)]
#[test]
fn ir_lowering_vector_number_mixed_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    fixture.assembly_options.include_outlined_code = true;
    let source = CString::new(
        r#"
local function foo(vectors: {vector}, i: string, t: {})
    return vectors[i] * (1 - t)
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
  CHECK_TAG R0, ttable, exit(entry)
  CHECK_TAG R1, tstring, exit(entry)
  CHECK_TAG R2, ttable, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  SET_SAVEDPC 1u
  GET_TABLE R4, R0, R1
  JUMP bb_fallback_3
bb_4:
  CHECK_TAG R4, tvector, exit(2)
  CHECK_TAG R5, tnumber, bb_fallback_5
  %24 = LOAD_TVALUE R4, 0i, tvector
  %25 = LOAD_DOUBLE R5
  %26 = NUM_TO_FLOAT %25
  %27 = FLOAT_TO_VEC %26
  %28 = MUL_VEC %24, %27
  %29 = TAG_VECTOR %28
  STORE_TVALUE R3, %29
  JUMP bb_6
bb_6:
  INTERRUPT 3u
  RETURN R3, 1i
bb_fallback_3:
  SET_SAVEDPC 2u
  DO_ARITH R5, K0 (1), R2, 9i
  JUMP bb_4
bb_fallback_5:
  SET_SAVEDPC 3u
  DO_ARITH R3, R4, R5, 10i
  JUMP bb_6
"#;

    assert_eq!(actual, expected);
}
