//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:469:ir_lowering_vector_mul_div_mixed`
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
//!   - translates_to -> rust_item ir_lowering_vector_mul_div_mixed

#[cfg(test)]
#[test]
fn ir_lowering_vector_mul_div_mixed() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function vec3combo(a: vector, b: vector, c: vector, d: vector)
    return a * 2 + b / 4 + 0.5 * c + 40 / d
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function vec3combo($arg0, $arg1, $arg2, $arg3) line 2
bb_0:
  CHECK_TAG R0, tvector, exit(entry)
  CHECK_TAG R1, tvector, exit(entry)
  CHECK_TAG R2, tvector, exit(entry)
  CHECK_TAG R3, tvector, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %12 = LOAD_TVALUE R0, 0i, tvector
  %14 = FLOAT_TO_VEC 2
  %15 = MUL_VEC %12, %14
  %20 = LOAD_TVALUE R1, 0i, tvector
  %22 = FLOAT_TO_VEC 4
  %23 = DIV_VEC %20, %22
  %32 = ADD_VEC %15, %23
  %37 = LOAD_TVALUE R2, 0i, tvector
  %39 = FLOAT_TO_VEC 0.5
  %40 = MUL_VEC %37, %39
  %49 = ADD_VEC %32, %40
  %55 = FLOAT_TO_VEC 40
  %56 = LOAD_TVALUE R3, 0i, tvector
  %57 = DIV_VEC %55, %56
  %66 = ADD_VEC %49, %57
  %67 = TAG_VECTOR %66
  STORE_TVALUE R4, %67
  INTERRUPT 7u
  RETURN R4, 1i
"#;

    assert_eq!(actual, expected);
}
