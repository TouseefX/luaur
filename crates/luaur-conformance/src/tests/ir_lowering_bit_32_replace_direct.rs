//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:4587:ir_lowering_bit_32_replace_direct`
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
//!   - translates_to -> rust_item ir_lowering_bit_32_replace_direct

#[cfg(test)]
#[test]
fn ir_lowering_bit_32_replace_direct() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: number, b: number)
    local x = bit32.band(a, 0x003FFFFF)
    local y = bit32.band(b, 0x003FFFFF)
    local z = bit32.replace(bit32.rshift(a, 22), bit32.rshift(b, 22), 10, 10)

    local v = vector.create(x, y, z)
    return v, v.x + v.y -- tests UINT_TO_FLOAT propagation as well
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
  %12 = BITAND_UINT %10, 4194303i
  %20 = LOAD_DOUBLE R1
  %21 = NUM_TO_UINT %20
  %23 = BITAND_UINT %21, 4194303i
  %33 = BITRSHIFT_UINT %10, 22i
  %43 = BITRSHIFT_UINT %21, 22i
  %78 = BITAND_UINT %33, -1047553i
  %79 = BITAND_UINT %43, 1023i
  %80 = BITLSHIFT_UINT %79, 10i
  %81 = BITOR_UINT %78, %80
  %96 = UINT_TO_FLOAT %12
  %97 = UINT_TO_FLOAT %23
  %98 = UINT_TO_FLOAT %81
  STORE_VECTOR R5, %96, %97, %98, tvector
  %102 = LOAD_TVALUE R5, 0i, tvector
  STORE_TVALUE R6, %102
  %107 = FLOAT_TO_NUM %96
  %113 = FLOAT_TO_NUM %97
  %122 = ADD_NUM %107, %113
  STORE_SPLIT_TVALUE R7, tnumber, %122
  INTERRUPT 48u
  RETURN R6, 2i
"#;

    assert_eq!(actual, expected);
}
