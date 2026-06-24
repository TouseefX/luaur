//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:7963:ir_lowering_type_alias_resolution`
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
//!   - translates_to -> rust_item ir_lowering_type_alias_resolution

#[cfg(test)]
#[test]
fn ir_lowering_type_alias_resolution() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _luau_compile_type_alias = ScopedFastFlag::new(&FFlag::LuauCompileTypeAliases, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
type foo = number
type bar = foo

local function meow(foo: foo, bar: bar)
  return foo + bar
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, false)
    );
    let expected = r#"
; function meow($arg0, $arg1) line 5
; R0: number [argument]
; R1: number [argument]
bb_0:
  CHECK_TAG R0, tnumber, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %10 = LOAD_DOUBLE R0
  %12 = ADD_NUM %10, R1
  STORE_DOUBLE R2, %12
  STORE_TAG R2, tnumber
  INTERRUPT 1u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);

    let source = CString::new(
        r#"
type foo = bar
type bar = foo

local function meow(foo: foo, bar: bar)
  return foo + bar
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, false)
    );
    let expected = r#"
; function meow($arg0, $arg1) line 5
bb_bytecode_0:
  CHECK_TAG R0, tnumber, bb_fallback_1
  CHECK_TAG R1, tnumber, bb_fallback_1
  %4 = LOAD_DOUBLE R0
  %6 = ADD_NUM %4, R1
  STORE_DOUBLE R2, %6
  STORE_TAG R2, tnumber
  JUMP bb_2
bb_2:
  INTERRUPT 1u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
