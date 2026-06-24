//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:7698:ir_lowering_vec_op_reuse`
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
//!   - translates_to -> rust_item ir_lowering_vec_op_reuse

#[cfg(test)]
#[test]
fn ir_lowering_vec_op_reuse() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(c: vector): vector
    return vector.create(
        math.sin(3.0 * vector.magnitude(c) + 6.0),
        math.sin(3.0 * vector.magnitude(c) + 1.0),
        math.sin(3.0 * vector.magnitude(c) + 2.0)
    )
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
  implicit CHECK_SAFE_ENV exit(0)
  %7 = LOAD_TVALUE R0, 0i, tvector
  %8 = DOT_VEC %7, %7
  %9 = SQRT_FLOAT %8
  %10 = FLOAT_TO_NUM %9
  %17 = MUL_NUM %10, 3
  %23 = ADD_NUM %17, 6
  %30 = INVOKE_LIBM 24u, %23
  %53 = ADD_NUM %17, 1
  %60 = INVOKE_LIBM 24u, %53
  %83 = ADD_NUM %17, 2
  %90 = INVOKE_LIBM 24u, %83
  %104 = NUM_TO_FLOAT %30
  %105 = NUM_TO_FLOAT %60
  %106 = NUM_TO_FLOAT %90
  STORE_VECTOR R1, %104, %105, %106
  STORE_TAG R1, tvector
  INTERRUPT 37u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
