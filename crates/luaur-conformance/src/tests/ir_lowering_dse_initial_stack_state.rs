//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:649:ir_lowering_dse_initial_stack_state`
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
//!   - translates_to -> rust_item ir_lowering_dse_initial_stack_state

#[cfg(test)]
#[test]
fn ir_lowering_dse_initial_stack_state() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo()
    while {} do
        local _ = not _,{}
        _ = nil
    end
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo() line 2
bb_bytecode_0:
  SET_SAVEDPC 1u
  %1 = NEW_TABLE 0u, 0u
  STORE_POINTER R0, %1
  STORE_TAG R0, ttable
  CHECK_GC
  JUMP bb_2
bb_2:
  implicit CHECK_SAFE_ENV exit(3)
  GET_CACHED_IMPORT R1, K1 (nil), 1073741824u ('_'), 4u
  SET_SAVEDPC 7u
  %14 = NEW_TABLE 0u, 0u
  STORE_POINTER R1, %14
  STORE_TAG R1, ttable
  STORE_TAG R0, tnil
  INTERRUPT 9u
  JUMP bb_bytecode_0
"#;

    assert_eq!(actual, expected);
}
