//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:6189:ir_lowering_linear_and_or`
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
//!   - translates_to -> rust_item ir_lowering_linear_and_or

#[cfg(test)]
#[test]
fn ir_lowering_linear_and_or() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a, b)
    return a and b, a or b
end
local function bar()
    local a, b = foo(3, 4)
    return a, b
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
  %0 = LOAD_TVALUE R0
  %1 = LOAD_TVALUE R1
  %2 = SELECT_IF_TRUTHY %0, %1, %0
  STORE_TVALUE R2, %2
  %6 = SELECT_IF_TRUTHY %0, %0, %1
  STORE_TVALUE R3, %6
  INTERRUPT 2u
  RETURN R2, 2i
; function bar() line 5
bb_bytecode_0:
  STORE_DOUBLE R0, 4
  STORE_TAG R0, tnumber
  STORE_DOUBLE R1, 3
  STORE_TAG R1, tnumber
  INTERRUPT 2u
  RETURN R0, 2i
"#;

    assert_eq!(actual, expected);
}
