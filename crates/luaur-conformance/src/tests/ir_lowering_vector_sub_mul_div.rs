//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:405:ir_lowering_vector_sub_mul_div`
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
//!   - translates_to -> rust_item ir_lowering_vector_sub_mul_div

#[cfg(test)]
#[test]
fn ir_lowering_vector_sub_mul_div() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function vec3combo(a: vector, b: vector, c: vector, d: vector)
    return a * b - c / d
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
  %14 = LOAD_TVALUE R0, 0i, tvector
  %15 = LOAD_TVALUE R1, 0i, tvector
  %16 = MUL_VEC %14, %15
  %23 = LOAD_TVALUE R2, 0i, tvector
  %24 = LOAD_TVALUE R3, 0i, tvector
  %25 = DIV_VEC %23, %24
  %34 = SUB_VEC %16, %25
  %35 = TAG_VECTOR %34
  STORE_TVALUE R4, %35
  INTERRUPT 3u
  RETURN R4, 1i
"#;

    assert_eq!(actual, expected);
}
