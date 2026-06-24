//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:6144:ir_lowering_optional_or`
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
//!   - translates_to -> rust_item ir_lowering_optional_or

#[cfg(test)]
#[test]
fn ir_lowering_optional_or() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a, b)
    a = a or 0
    b = b or 0
    return a + b
end
-- when a function like 'foo' is inlined, those 'default values' collapse
local function bar()
    return foo(3, 4)
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
  %1 = LOAD_TVALUE K0 (0)
  %2 = SELECT_IF_TRUTHY %0, %0, %1
  STORE_TVALUE R0, %2
  %4 = LOAD_TVALUE R1
  %5 = LOAD_TVALUE K0 (0)
  %6 = SELECT_IF_TRUTHY %4, %4, %5
  STORE_TVALUE R1, %6
  CHECK_TAG R0, tnumber, bb_fallback_1
  CHECK_TAG R1, tnumber, bb_fallback_1
  %12 = LOAD_DOUBLE R0
  %14 = ADD_NUM %12, R1
  STORE_DOUBLE R2, %14
  STORE_TAG R2, tnumber
  JUMP bb_2
bb_2:
  INTERRUPT 3u
  RETURN R2, 1i
; function bar() line 8
bb_bytecode_0:
  STORE_DOUBLE R0, 7
  STORE_TAG R0, tnumber
  INTERRUPT 5u
  RETURN R0, 1i
"#;

    assert_eq!(actual, expected);
}
