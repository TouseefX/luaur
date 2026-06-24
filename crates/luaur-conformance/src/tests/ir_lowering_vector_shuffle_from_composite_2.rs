//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:4871:ir_lowering_vector_shuffle_from_composite_2`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function isSupported (CodeGen/src/CodeGen.cpp)
//!   - calls -> method LoweringFixture::getCodegenAssembly (tests/IrLowering.test.cpp)
//!   - translates_to -> rust_item ir_lowering_vector_shuffle_from_composite_2

#[cfg(test)]
#[test]
fn ir_lowering_vector_shuffle_from_composite_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use std::ffi::CString;

    if luau_codegen_supported() == 0 {
        return;
    }

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(v: vertex)
    return v.uv.X * v.uv.Y
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function test($arg0) line 2
bb_0:
  CHECK_TAG R0, tuserdata, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %6 = LOAD_POINTER R0
  CHECK_USERDATA_TAG %6, 13i, exit(0)
  %8 = BUFFER_READF32 %6, 24i, tuserdata
  %9 = BUFFER_READF32 %6, 28i, tuserdata
  CHECK_GC
  %21 = FLOAT_TO_NUM %8
  %41 = FLOAT_TO_NUM %9
  %50 = MUL_NUM %21, %41
  STORE_DOUBLE R1, %50
  STORE_TAG R1, tnumber
  INTERRUPT 9u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
