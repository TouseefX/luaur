//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:6003:ir_lowering_bit_32_has_to_use_double_temporaries_add`
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
//!   - translates_to -> rust_item ir_lowering_bit_32_has_to_use_double_temporaries_add

#[cfg(test)]
#[test]
fn ir_lowering_bit_32_has_to_use_double_temporaries_add() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: number, b: number)
    local a = bit32.band(bit32.bor(a, 0) + 0.75, 0xffff)
    local b = bit32.band(bit32.bor(a, 0) + 1e30, 0xffff)
    local c = bit32.band(1e30 + bit32.bor(a, 1), 0xffff)
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
  %13 = UINT_TO_NUM %10
  %20 = ADD_NUM %13, 0.75
  %27 = NUM_TO_UINT %20
  %29 = BITAND_UINT %27, 65535i
  %30 = UINT_TO_NUM %29
  STORE_DOUBLE R2, %30
  STORE_TAG R2, tnumber
  %48 = ADD_NUM %30, 1e+30
  %55 = NUM_TO_UINT %48
  %57 = BITAND_UINT %55, 65535i
  %58 = UINT_TO_NUM %57
  STORE_SPLIT_TVALUE R3, tnumber, %58
  %68 = BITOR_UINT %29, 1i
  %69 = UINT_TO_NUM %68
  %76 = ADD_NUM %69, 1e+30
  %83 = NUM_TO_UINT %76
  %85 = BITAND_UINT %83, 65535i
  %86 = UINT_TO_NUM %85
  STORE_SPLIT_TVALUE R4, tnumber, %86
  INTERRUPT 42u
  RETURN R2, 3i
"#;

    assert_eq!(actual, expected);
}
