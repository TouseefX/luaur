//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:4716:ir_lowering_bit_32_single_arg_btest`
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
//!   - translates_to -> rust_item ir_lowering_bit_32_single_arg_btest

#[cfg(test)]
#[test]
fn ir_lowering_bit_32_single_arg_btest() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: number)
    return bit32.btest(a)
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
  CHECK_TAG R0, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %7 = LOAD_DOUBLE R0
  %8 = NUM_TO_UINT %7
  %9 = CMP_INT %8, 0i, not_eq
  STORE_INT R1, %9
  STORE_TAG R1, tboolean
  INTERRUPT 5u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
