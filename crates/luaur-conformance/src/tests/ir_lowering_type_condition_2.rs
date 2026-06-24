//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1077:ir_lowering_type_condition_2`
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
//!   - translates_to -> rust_item ir_lowering_type_condition_2

#[cfg(test)]
#[test]
fn ir_lowering_type_condition_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a, b)
    if type(a) == "number" and type(b) == "number" then
        return a + b
    end
    return nil
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
  JUMP bb_4
bb_4:
  JUMP_EQ_TAG R0, tnumber, bb_3, bb_bytecode_1
bb_3:
  implicit CHECK_SAFE_ENV exit(7)
  JUMP bb_7
bb_7:
  JUMP_EQ_TAG R1, tnumber, bb_6, bb_bytecode_1
bb_6:
  CHECK_TAG R0, tnumber, bb_fallback_8
  CHECK_TAG R1, tnumber, bb_fallback_8
  %26 = LOAD_DOUBLE R0
  %28 = ADD_NUM %26, R1
  STORE_DOUBLE R2, %28
  STORE_TAG R2, tnumber
  JUMP bb_9
bb_9:
  INTERRUPT 15u
  RETURN R2, 1i
bb_bytecode_1:
  STORE_TAG R2, tnil
  INTERRUPT 17u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
