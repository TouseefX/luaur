//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:2095:ir_lowering_explicit_upvalue_and_local_types`
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
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item ir_lowering_explicit_upvalue_and_local_types

#[cfg(test)]
#[test]
fn ir_lowering_explicit_upvalue_and_local_types() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local y: vector = ...

local function getsum(t)
    local x: vector = t
    return x.X + x.Y + y.X + y.Y
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, false)
    );
    let expected = r#"
; function getsum($arg0) line 4
; U0: vector
; R0: vector from 0 to 14
bb_bytecode_0:
  CHECK_TAG R0, tvector, exit(0)
  %2 = LOAD_FLOAT R0, 0i
  %3 = FLOAT_TO_NUM %2
  %8 = LOAD_FLOAT R0, 4i
  %9 = FLOAT_TO_NUM %8
  %18 = ADD_NUM %3, %9
  %21 = GET_UPVALUE U0
  STORE_TVALUE R4, %21
  CHECK_TAG R4, tvector, bb_exit_1
   ; exit sync: R5, R3, {%9, %18}
  %25 = EXTRACT_VEC %21, 0i
  %26 = FLOAT_TO_NUM %25
  %35 = ADD_NUM %18, %26
  STORE_TVALUE R3, %21
  %42 = EXTRACT_VEC %21, 1i
  %43 = FLOAT_TO_NUM %42
  %52 = ADD_NUM %35, %43
  STORE_DOUBLE R1, %52
  STORE_TAG R1, tnumber
  INTERRUPT 13u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
