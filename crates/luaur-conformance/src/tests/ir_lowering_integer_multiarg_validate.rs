//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:8035:ir_lowering_integer_multiarg_validate`
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
//!   - translates_to -> rust_item ir_lowering_integer_multiarg_validate

#[cfg(test)]
#[test]
fn ir_lowering_integer_multiarg_validate() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _luau_integer_fastcalls = ScopedFastFlag::new(&FFlag::LuauIntegerFastcalls, true);
    let _luau_codegen_integer_2 = ScopedFastFlag::new(&FFlag::LuauCodegenInteger2, true);
    let _luau_integer_type = ScopedFastFlag::new(&FFlag::LuauIntegerType2, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function f(a, b)
    return integer.bxor(a, b, a)
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function f($arg0, $arg1) line 2
bb_bytecode_0:
  implicit CHECK_SAFE_ENV exit(0)
  CHECK_TAG R0, tinteger, exit(2)
  CHECK_TAG R1, tinteger, exit(2)
  %7 = LOAD_INT64 R0
  %8 = LOAD_INT64 R1
  %9 = BITXOR_INT64 %7, %8
  %11 = BITXOR_INT64 %9, %7
  STORE_INT64 R2, %11
  STORE_TAG R2, tinteger
  INTERRUPT 8u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
