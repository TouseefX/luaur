//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1371:ir_lowering_vector_custom_access_chain`
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
//!   - translates_to -> rust_item ir_lowering_vector_custom_access_chain

#[cfg(test)]
#[test]
fn ir_lowering_vector_custom_access_chain() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: vector, b: vector)
    return a.Unit * b.Magnitude
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
  %8 = LOAD_FLOAT R0, 0i
  %9 = LOAD_FLOAT R0, 4i
  %10 = LOAD_FLOAT R0, 8i
  %11 = MUL_FLOAT %8, %8
  %12 = MUL_FLOAT %9, %9
  %13 = MUL_FLOAT %10, %10
  %14 = ADD_FLOAT %11, %12
  %15 = ADD_FLOAT %14, %13
  %16 = SQRT_FLOAT %15
  %17 = DIV_FLOAT 1, %16
  %18 = MUL_FLOAT %8, %17
  %19 = MUL_FLOAT %9, %17
  %20 = MUL_FLOAT %10, %17
  STORE_VECTOR R3, %18, %19, %20
  STORE_TAG R3, tvector
  %25 = LOAD_FLOAT R1, 0i
  %26 = LOAD_FLOAT R1, 4i
  %27 = LOAD_FLOAT R1, 8i
  %28 = MUL_FLOAT %25, %25
  %29 = MUL_FLOAT %26, %26
  %30 = MUL_FLOAT %27, %27
  %31 = ADD_FLOAT %28, %29
  %32 = ADD_FLOAT %31, %30
  %33 = SQRT_FLOAT %32
  %41 = LOAD_TVALUE R3, 0i, tvector
  %44 = FLOAT_TO_VEC %33
  %45 = MUL_VEC %41, %44
  %46 = TAG_VECTOR %45
  STORE_TVALUE R2, %46
  INTERRUPT 5u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
