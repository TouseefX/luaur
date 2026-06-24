//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:4995:ir_lowering_comparison_propagation_wall`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item ir_lowering_comparison_propagation_wall

#[cfg(test)]
#[test]
fn ir_lowering_comparison_propagation_wall() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a, b)
    local x = type(b)
    local y = (not a) ~= b
    local z = type(b)
    return x, y, z
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
bb_bytecode_0:
  implicit CHECK_SAFE_ENV exit(0)
  %1 = LOAD_TAG R1
  %2 = GET_TYPE %1
  STORE_POINTER R2, %2
  STORE_TAG R2, tstring
  %6 = LOAD_TAG R0
  %7 = LOAD_INT R0
  %8 = NOT_ANY %6, %7
  STORE_INT R4, %8
  STORE_TAG R4, tboolean
  SET_SAVEDPC 7u
  %12 = CMP_ANY R4, R1, eq
  %13 = SUB_INT 1i, %12
  STORE_INT R3, %13
  STORE_TAG R3, tboolean
  JUMP bb_bytecode_2
bb_bytecode_2:
  implicit CHECK_SAFE_ENV exit(10)
  %21 = LOAD_TAG R1
  %22 = GET_TYPE %21
  STORE_POINTER R4, %22
  STORE_TAG R4, tstring
  INTERRUPT 15u
  RETURN R2, 3i
"#;

    assert_eq!(actual, expected);
}
