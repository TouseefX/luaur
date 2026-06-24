//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:7743:ir_lowering_vec_op_reuse_2`
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
//!   - translates_to -> rust_item ir_lowering_vec_op_reuse_2

#[cfg(test)]
#[test]
fn ir_lowering_vec_op_reuse_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(c: vector, d: vector): vector
    return {2 * c + d, 2 * c + d}
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1) line 2
bb_0:
  CHECK_TAG R0, tvector, exit(entry)
  CHECK_TAG R1, tvector, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  SET_SAVEDPC 1u
  %7 = NEW_TABLE 2u, 0u
  STORE_POINTER R2, %7
  STORE_TAG R2, ttable
  CHECK_GC
  %13 = LOAD_TVALUE R0, 0i, tvector
  %15 = FLOAT_TO_VEC 2
  %16 = MUL_VEC %13, %15
  %24 = LOAD_TVALUE R1, 0i, tvector
  %25 = ADD_VEC %16, %24
  %26 = TAG_VECTOR %25
  STORE_TVALUE R3, %26
  STORE_TVALUE R4, %26
  SETLIST 6u, R2, R3, 2i, 1u, 2u
  INTERRUPT 8u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
