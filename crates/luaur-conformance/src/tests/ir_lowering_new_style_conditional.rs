//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:6265:ir_lowering_new_style_conditional`
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
//!   - calls -> method Lexer::current (Ast/include/Luau/Lexer.h)
//!   - calls -> method LoweringFixture::getCodegenAssembly (tests/IrLowering.test.cpp)
//!   - translates_to -> rust_item ir_lowering_new_style_conditional

#[cfg(test)]
#[test]
fn ir_lowering_new_style_conditional() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: boolean, b: number, c: number)
    local x = if a then b else c
    return x + 1
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
  CHECK_TAG R0, tboolean, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  CHECK_TAG R2, tnumber, exit(entry)
  JUMP bb_4
bb_4:
  JUMP bb_bytecode_1
bb_bytecode_1:
  JUMP_IF_FALSY R0, bb_bytecode_2, bb_5
bb_5:
  %9 = LOAD_TVALUE R1, 0i, tnumber
  STORE_TVALUE R3, %9
  JUMP bb_bytecode_3
bb_bytecode_2:
  %12 = LOAD_TVALUE R2, 0i, tnumber
  STORE_TVALUE R3, %12
  JUMP bb_bytecode_3
bb_bytecode_3:
  CHECK_TAG R3, tnumber, exit(4)
  %17 = LOAD_DOUBLE R3
  %18 = ADD_NUM %17, 1
  STORE_DOUBLE R4, %18
  STORE_TAG R4, tnumber
  INTERRUPT 5u
  RETURN R4, 1i
"#;

    assert_eq!(actual, expected);
}
