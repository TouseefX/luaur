//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3448:ir_lowering_argument_type_refinement`
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
//!   - translates_to -> rust_item ir_lowering_argument_type_refinement

#[cfg(test)]
#[test]
fn ir_lowering_argument_type_refinement() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function getsum(x, y)
    x = vector(1, y, 3)
    return x.Y + x.Z
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, false)
    );
    let expected = r#"
; function getsum($arg0, $arg1) line 2
; R0: vector [argument]
bb_bytecode_0:
  implicit CHECK_SAFE_ENV exit(0)
  CHECK_TAG R1, tnumber, bb_exit_2
   ; exit sync: R5, R3, {}
  %12 = LOAD_DOUBLE R1
  %15 = NUM_TO_FLOAT %12
  STORE_VECTOR R2, 1, %15, 3
  STORE_TAG R2, tvector
  %25 = FLOAT_TO_NUM %15
  %40 = ADD_NUM %25, 3
  STORE_DOUBLE R2, %40
  STORE_TAG R2, tnumber
  INTERRUPT 14u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
