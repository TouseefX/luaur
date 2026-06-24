//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:578:ir_lowering_vector_floor_ceil_abs`
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
//!   - translates_to -> rust_item ir_lowering_vector_floor_ceil_abs

#[cfg(test)]
#[test]
fn ir_lowering_vector_floor_ceil_abs() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function vecops(a: vector)
    return vector.abs(a), vector.floor(a), vector.ceil(a)
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function vecops($arg0) line 2
bb_0:
  CHECK_TAG R0, tvector, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %7 = LOAD_TVALUE R0, 0i, tvector
  %8 = ABS_VEC %7
  %9 = TAG_VECTOR %8
  STORE_TVALUE R1, %9
  %16 = FLOOR_VEC %7
  %17 = TAG_VECTOR %16
  STORE_TVALUE R2, %17
  %24 = CEIL_VEC %7
  %25 = TAG_VECTOR %24
  STORE_TVALUE R3, %25
  INTERRUPT 15u
  RETURN R1, 3i
"#;

    assert_eq!(actual, expected);
}
