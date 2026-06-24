//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1122:ir_lowering_assert_type_guard`
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
//!   - translates_to -> rust_item ir_lowering_assert_type_guard

#[cfg(test)]
#[test]
fn ir_lowering_assert_type_guard() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a)
    assert(type(a) == "number")
    return a * 2
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0) line 2
bb_bytecode_0:
  implicit CHECK_SAFE_ENV exit(0)
  %1 = LOAD_TAG R0
  %2 = GET_TYPE %1
  STORE_POINTER R3, %2
  STORE_TAG R3, tstring
  %9 = CMP_TAG %1, tnumber, eq
  STORE_TAG R2, tboolean
  STORE_INT R2, %9
  JUMP bb_bytecode_2
bb_bytecode_2:
  CHECK_TRUTHY tboolean, %9, exit(10)
  JUMP bb_5
bb_5:
  CHECK_TAG %1, tnumber, bb_fallback_6
  %30 = LOAD_DOUBLE R0
  %31 = ADD_NUM %30, %30
  STORE_DOUBLE R1, %31
  STORE_TAG R1, tnumber
  JUMP bb_7
bb_7:
  INTERRUPT 14u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
