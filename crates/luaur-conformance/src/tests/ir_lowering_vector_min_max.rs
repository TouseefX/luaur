//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:547:ir_lowering_vector_min_max`
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
//!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
//!   - translates_to -> rust_item ir_lowering_vector_min_max

#[cfg(test)]
#[test]
fn ir_lowering_vector_min_max() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function vecops(a: vector, b: vector)
    return vector.min(a, b), vector.max(a, b)
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function vecops($arg0, $arg1) line 2
bb_0:
  CHECK_TAG R0, tvector, exit(entry)
  CHECK_TAG R1, tvector, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %11 = LOAD_TVALUE R0, 0i, tvector
  %12 = LOAD_TVALUE R1, 0i, tvector
  %13 = MIN_VEC %12, %11
  %14 = TAG_VECTOR %13
  STORE_TVALUE R2, %14
  %24 = MAX_VEC %12, %11
  %25 = TAG_VECTOR %24
  STORE_TVALUE R3, %25
  INTERRUPT 14u
  RETURN R2, 2i
"#;

    assert_eq!(actual, expected);
}
