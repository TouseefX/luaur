//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:6890:ir_lowering_upvalue_access_load_store_1`
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
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item ir_lowering_upvalue_access_load_store_1

#[cfg(test)]
#[test]
fn ir_lowering_upvalue_access_load_store_1() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local m = 1

local function foo(a: number, b: number)
    return m * a + m * b
end

function setm(x) m = x end
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
  %6 = GET_UPVALUE U0
  STORE_TVALUE R4, %6
  CHECK_TAG R4, tnumber, exit(1)
  %12 = LOAD_DOUBLE R4
  %14 = MUL_NUM %12, R0
  %25 = MUL_NUM %12, R1
  %34 = ADD_NUM %14, %25
  STORE_DOUBLE R2, %34
  STORE_TAG R2, tnumber
  INTERRUPT 5u
  RETURN R2, 1i
; function setm($arg0) line 8
bb_bytecode_0:
  %0 = LOAD_TVALUE R0
  SET_UPVALUE U0, %0, undef
  INTERRUPT 1u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
