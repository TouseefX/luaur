//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:511:ir_lowering_vector_lerp`
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
//!   - translates_to -> rust_item ir_lowering_vector_lerp

#[cfg(test)]
#[test]
fn ir_lowering_vector_lerp() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function vec3lerp(a: vector, b: vector, t: number)
    return vector.lerp(a, b, t)
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function vec3lerp($arg0, $arg1, $arg2) line 2
bb_0:
  CHECK_TAG R0, tvector, exit(entry)
  CHECK_TAG R1, tvector, exit(entry)
  CHECK_TAG R2, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %15 = LOAD_TVALUE R0, 0i, tvector
  %16 = LOAD_TVALUE R1, 0i, tvector
  %17 = LOAD_DOUBLE R2
  %18 = NUM_TO_FLOAT %17
  %19 = FLOAT_TO_VEC %18
  %20 = FLOAT_TO_VEC 1
  %21 = SUB_VEC %16, %15
  %22 = MULADD_VEC %21, %19, %15
  %23 = SELECT_VEC %22, %16, %19, %20
  %24 = TAG_VECTOR %23
  STORE_TVALUE R3, %24
  INTERRUPT 8u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
