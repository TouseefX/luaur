//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:6932:ir_lowering_upvalue_access_load_store_2`
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
//!   - calls -> macro upvalue (VM/src/lobject.h)
//!   - calls -> method LoweringFixture::getCodegenAssembly (tests/IrLowering.test.cpp)
//!   - translates_to -> rust_item ir_lowering_upvalue_access_load_store_2

#[cfg(test)]
#[test]
fn ir_lowering_upvalue_access_load_store_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local m

local function foo(a: number, b: number)
    m = a - b
    m = m * a + m * b
    return m + a
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1) line 4
bb_0:
  CHECK_TAG R0, tnumber, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %10 = LOAD_DOUBLE R0
  %11 = LOAD_DOUBLE R1
  %12 = SUB_NUM %10, %11
  STORE_DOUBLE R2, %12
  STORE_TAG R2, tnumber
  %15 = LOAD_TVALUE R2, 0i, tnumber
  SET_UPVALUE U0, %15, tnumber
  %25 = MUL_NUM %12, %10
  %36 = MUL_NUM %12, %11
  %45 = ADD_NUM %25, %36
  STORE_DOUBLE R2, %45
  %48 = LOAD_TVALUE R2, 0i, tnumber
  SET_UPVALUE U0, %48, tnumber
  %58 = ADD_NUM %45, %10
  STORE_DOUBLE R2, %58
  INTERRUPT 10u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
