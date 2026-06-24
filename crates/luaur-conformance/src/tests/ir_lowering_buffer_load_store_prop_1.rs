//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:7151:ir_lowering_buffer_load_store_prop_1`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item ir_lowering_buffer_load_store_prop_1

#[cfg(test)]
#[test]
fn ir_lowering_buffer_load_store_prop_1() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(b: buffer)
    return buffer.readf32(b, 0) * buffer.readf32(b, 0) + buffer.readf32(b, 4) * buffer.readf32(b, 4)
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
  CHECK_TAG R0, tbuffer, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %7 = LOAD_POINTER R0
  CHECK_BUFFER_LEN %7, 0i, 0i, 8i, undef, exit(2)
  %10 = BUFFER_READF32 %7, 0i, tbuffer
  %11 = FLOAT_TO_NUM %10
  %32 = MUL_NUM %11, %11
  %41 = BUFFER_READF32 %7, 4i, tbuffer
  %42 = FLOAT_TO_NUM %41
  %63 = MUL_NUM %42, %42
  %72 = ADD_NUM %32, %63
  STORE_DOUBLE R1, %72
  STORE_TAG R1, tnumber
  INTERRUPT 31u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
