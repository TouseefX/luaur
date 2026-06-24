//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:8238:ir_lowering_integer_fastcall_constant`
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
//!   - translates_to -> rust_item ir_lowering_integer_fastcall_constant

#[cfg(test)]
#[test]
fn ir_lowering_integer_fastcall_constant() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _luau_integer_fastcalls = ScopedFastFlag::new(&FFlag::LuauIntegerFastcalls, true);
    let _luau_codegen_integer_2 = ScopedFastFlag::new(&FFlag::LuauCodegenInteger2, true);
    let _luau_codegen_integer_fastcall_2k =
        ScopedFastFlag::new(&FFlag::LuauCodegenIntegerFastcall2k, true);
    let _luau_integer_type = ScopedFastFlag::new(&FFlag::LuauIntegerType2, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(x: integer)
    return integer.band(x, 5i)
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0) line 2
; R0: integer [argument]
bb_0:
  CHECK_TAG R0, tinteger, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %7 = LOAD_INT64 R0
  %8 = BITAND_INT64 %7, 5i
  STORE_INT64 R1, %8
  STORE_TAG R1, tinteger
  INTERRUPT 7u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
