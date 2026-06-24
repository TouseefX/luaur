//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1260:ir_lowering_vector_custom_access`
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
//!   - translates_to -> rust_item ir_lowering_vector_custom_access

#[cfg(test)]
#[test]
fn ir_lowering_vector_custom_access() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function vec3magn(a: vector)
    return a.Magnitude * 3
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function vec3magn($arg0) line 2
bb_0:
  CHECK_TAG R0, tvector, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %6 = LOAD_FLOAT R0, 0i
  %7 = LOAD_FLOAT R0, 4i
  %8 = LOAD_FLOAT R0, 8i
  %9 = MUL_FLOAT %6, %6
  %10 = MUL_FLOAT %7, %7
  %11 = MUL_FLOAT %8, %8
  %12 = ADD_FLOAT %9, %10
  %13 = ADD_FLOAT %12, %11
  %14 = SQRT_FLOAT %13
  %15 = FLOAT_TO_NUM %14
  %21 = MUL_NUM %15, 3
  STORE_DOUBLE R1, %21
  STORE_TAG R1, tnumber
  INTERRUPT 3u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
