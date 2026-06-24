//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1594:ir_lowering_vector_library_chain`
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
//!   - translates_to -> rust_item ir_lowering_vector_library_chain

#[cfg(test)]
#[test]
fn ir_lowering_vector_library_chain() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: vector, b: vector)
    return vector.normalize(a) * (vector.magnitude(b) + vector.dot(a, b))
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
  implicit CHECK_SAFE_ENV exit(0)
  %9 = LOAD_TVALUE R0, 0i, tvector
  %10 = DOT_VEC %9, %9
  %11 = SQRT_FLOAT %10
  %12 = DIV_FLOAT 1, %11
  %13 = FLOAT_TO_VEC %12
  %14 = MUL_VEC %9, %13
  %21 = LOAD_TVALUE R1, 0i, tvector
  %22 = DOT_VEC %21, %21
  %23 = SQRT_FLOAT %22
  %24 = FLOAT_TO_NUM %23
  %35 = DOT_VEC %9, %21
  %36 = FLOAT_TO_NUM %35
  %46 = ADD_NUM %24, %36
  %55 = NUM_TO_FLOAT %46
  %56 = FLOAT_TO_VEC %55
  %57 = MUL_VEC %14, %56
  %58 = TAG_VECTOR %57
  STORE_TVALUE R2, %58
  INTERRUPT 19u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
