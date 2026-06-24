//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:6975:ir_lowering_upvalue_access_load_store_3`
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
//!   - translates_to -> rust_item ir_lowering_upvalue_access_load_store_3

#[cfg(test)]
#[test]
fn ir_lowering_upvalue_access_load_store_3() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local m = 1

local function foo()
    local a = m
    m = a
    local b = m
    m = b
    return m + a + b
end

function setm(x, y) m = x end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo() line 4
bb_bytecode_0:
  %0 = GET_UPVALUE U0
  SET_UPVALUE U0, %0, undef
  SET_UPVALUE U0, %0, undef
  STORE_TVALUE R4, %0
  CHECK_TAG R4, tnumber, bb_exit_1
   ; exit sync: R1, R0, {%0}
  %14 = LOAD_DOUBLE R4
  %16 = ADD_NUM %14, %14
  %25 = ADD_NUM %16, %14
  STORE_DOUBLE R2, %25
  STORE_TAG R2, tnumber
  INTERRUPT 7u
  RETURN R2, 1i
; function setm($arg0, $arg1) line 12
bb_bytecode_0:
  %0 = LOAD_TVALUE R0
  SET_UPVALUE U0, %0, undef
  INTERRUPT 1u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
