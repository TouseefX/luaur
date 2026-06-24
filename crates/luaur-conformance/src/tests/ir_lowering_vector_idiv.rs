//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1636:ir_lowering_vector_idiv`
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
//!   - translates_to -> rust_item ir_lowering_vector_idiv

#[cfg(test)]
#[test]
fn ir_lowering_vector_idiv() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(x: vector): vector
    x *= 1.5
    x -= x // 1
    x -= vector.create(0.5, 0.5, 0.5)
    return x
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
bb_0:
  CHECK_TAG R0, tvector, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %6 = LOAD_TVALUE R0, 0i, tvector
  %8 = FLOAT_TO_VEC 1.5
  %9 = MUL_VEC %6, %8
  %16 = FLOAT_TO_VEC 1
  %17 = IDIV_VEC %9, %16
  %26 = SUB_VEC %9, %17
  %29 = LOAD_TVALUE K2 (0.5, 0.5, 0.5), 0i, tvector
  %37 = SUB_VEC %26, %29
  %38 = TAG_VECTOR %37
  STORE_TVALUE R0, %38
  INTERRUPT 5u
  RETURN R0, 1i
"#;

    assert_eq!(actual, expected);
}
