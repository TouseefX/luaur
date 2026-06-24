//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:4502:ir_lowering_library_field_types_and_constants_c_api`
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
//!   - calls -> method LoweringFixture::getCodegenAssemblyUsingCApi (tests/IrLowering.test.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - translates_to -> rust_item ir_lowering_library_field_types_and_constants_c_api

#[cfg(test)]
#[test]
fn ir_lowering_library_field_types_and_constants_c_api() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo()
    return test.some_nil, test.some_boolean, test.some_number, test.some_vector, test.some_string
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly_using_c_api(source.as_ptr(), true, 1)
    );
    let expected = r#"
; function foo() line 2
bb_bytecode_0:
  STORE_TAG R0, tnil
  STORE_INT R1, 1i
  STORE_TAG R1, tboolean
  STORE_DOUBLE R2, 4.75
  STORE_TAG R2, tnumber
  %5 = LOAD_TVALUE K1 (1, 2, 4), 0i, tvector
  STORE_TVALUE R3, %5
  %7 = LOAD_TVALUE K2 ('test'), 0i, tstring
  STORE_TVALUE R4, %7
  INTERRUPT 5u
  RETURN R0, 5i
"#;

    assert_eq!(actual, expected);
}
