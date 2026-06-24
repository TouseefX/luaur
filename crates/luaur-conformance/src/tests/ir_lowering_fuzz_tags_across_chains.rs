//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:6374:ir_lowering_fuzz_tags_across_chains`
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
//!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
//!   - translates_to -> rust_item ir_lowering_fuzz_tags_across_chains

#[cfg(test)]
#[test]
fn ir_lowering_fuzz_tags_across_chains() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function f(...)
    if bit32.btest(538976288,4,4,4,262144) then
    elseif bit32.btest(538976288,4,_,4,67108864) then
    end
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 1, false)
    );
    let expected = r#"
; function f() line 2
bb_bytecode_0:
  implicit CHECK_SAFE_ENV exit(0)
  FALLBACK_PREPVARARGS 0u, 0i
  STORE_INT R0, 0i
  STORE_TAG R0, tboolean
  JUMP_IF_FALSY R0, bb_bytecode_1, bb_4
bb_4:
  INTERRUPT 11u
  RETURN R0, 0i
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(12)
  GET_CACHED_IMPORT R3, K6 (nil), 1078984704u ('_'), 15u
  CHECK_TAG R3, tnumber, bb_exit_6
   ; exit sync: R5, R4, R2, R1, {}
  STORE_INT R0, 0i
  STORE_TAG R0, tboolean
  JUMP_IF_FALSY R0, bb_bytecode_2, bb_bytecode_2
bb_bytecode_2:
  INTERRUPT 23u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
