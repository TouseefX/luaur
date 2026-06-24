//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:6051:ir_lowering_bit_32_no_double_temporaries_sub`
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
//!   - translates_to -> rust_item ir_lowering_bit_32_no_double_temporaries_sub

#[cfg(test)]
#[test]
fn ir_lowering_bit_32_no_double_temporaries_sub() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: number, b: number)
    local a = bit32.band(bit32.bor(a, 0) - bit32.bor(b, 0), 0xffff)
    local b = bit32.band(bit32.bor(a, 0) - 127, 0xffff)
    local c = bit32.band(254 - bit32.bor(a, 1), 0xffff)
    return a, b, c
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
  CHECK_TAG R0, tnumber, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %9 = LOAD_DOUBLE R0
  %10 = NUM_TO_UINT %9
  %20 = LOAD_DOUBLE R1
  %21 = NUM_TO_UINT %20
  %41 = SUB_INT %10, %21
  %43 = BITAND_UINT %41, 65535i
  %44 = UINT_TO_NUM %43
  STORE_DOUBLE R2, %44
  STORE_TAG R2, tnumber
  %69 = SUB_INT %43, 127i
  %71 = BITAND_UINT %69, 65535i
  %72 = UINT_TO_NUM %71
  STORE_SPLIT_TVALUE R3, tnumber, %72
  %82 = BITOR_UINT %43, 1i
  %97 = SUB_INT 254i, %82
  %99 = BITAND_UINT %97, 65535i
  %100 = UINT_TO_NUM %99
  STORE_SPLIT_TVALUE R4, tnumber, %100
  INTERRUPT 49u
  RETURN R2, 3i
"#;

    assert_eq!(actual, expected);
}
