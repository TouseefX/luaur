//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:7917:ir_lowering_collatz`
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
//!   - translates_to -> rust_item ir_lowering_collatz

#[cfg(test)]
#[test]
fn ir_lowering_collatz() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function collatz(x : number)
    return if ((x % 2) == 1) then 3 * x + 1 else x // 2
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, false)
    );
    let expected = r#"
; function collatz($arg0) line 2
; R0: number [argument]
bb_0:
  CHECK_TAG R0, tnumber, exit(entry)
  JUMP bb_3
bb_3:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %6 = LOAD_DOUBLE R0
  %7 = MOD_NUM %6, 2
  JUMP bb_5
bb_5:
  JUMP_CMP_NUM %7, 1, not_eq, bb_bytecode_2, bb_4
bb_4:
  %16 = LOAD_DOUBLE R0
  %17 = MUL_NUM %16, 3
  %23 = ADD_NUM %17, 1
  STORE_DOUBLE R1, %23
  STORE_TAG R1, tnumber
  INTERRUPT 5u
  RETURN R1, 1i
bb_bytecode_2:
  %30 = LOAD_DOUBLE R0
  %31 = IDIV_NUM %30, 2
  STORE_DOUBLE R1, %31
  STORE_TAG R1, tnumber
  INTERRUPT 7u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
