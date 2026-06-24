//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:4678:ir_lowering_bit_32_single_arg`
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
//!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
//!   - translates_to -> rust_item ir_lowering_bit_32_single_arg

#[cfg(test)]
#[test]
fn ir_lowering_bit_32_single_arg() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: number, b: number, c: number)
    return bit32.band(a) + bit32.bor(b) + bit32.bxor(c)
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1, $arg2) line 2
bb_0:
  CHECK_TAG R0, tnumber, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  CHECK_TAG R2, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %11 = LOAD_DOUBLE R0
  %12 = NUM_TO_UINT %11
  %13 = UINT_TO_NUM %12
  %20 = LOAD_DOUBLE R1
  %21 = NUM_TO_UINT %20
  %22 = UINT_TO_NUM %21
  %32 = ADD_NUM %13, %22
  %38 = LOAD_DOUBLE R2
  %39 = NUM_TO_UINT %38
  %40 = UINT_TO_NUM %39
  %50 = ADD_NUM %32, %40
  STORE_DOUBLE R3, %50
  STORE_TAG R3, tnumber
  INTERRUPT 17u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
