//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3261:ir_lowering_fastcall_type_infer_through_local`
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
//!   - translates_to -> rust_item ir_lowering_fastcall_type_infer_through_local

#[cfg(test)]
#[test]
fn ir_lowering_fastcall_type_infer_through_local() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function getsum(x, c)
    local v = vector(x, 2, 3)
    if c then
        return v.X + v.Y
    else
        return v.Z
    end
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
; R2: vector from 0 to 18
bb_bytecode_0:
  implicit CHECK_SAFE_ENV exit(0)
  CHECK_TAG R0, tnumber, bb_exit_4
   ; exit sync: R5, R4, {}
  %11 = LOAD_DOUBLE R0
  %14 = NUM_TO_FLOAT %11
  STORE_VECTOR R2, %14, 2, 3
  STORE_TAG R2, tvector
  JUMP_IF_FALSY R1, bb_bytecode_1, bb_3
bb_3:
  %23 = LOAD_FLOAT R2, 0i
  %24 = FLOAT_TO_NUM %23
  %29 = LOAD_FLOAT R2, 4i
  %30 = FLOAT_TO_NUM %29
  %39 = ADD_NUM %24, %30
  STORE_DOUBLE R3, %39
  STORE_TAG R3, tnumber
  INTERRUPT 14u
  RETURN R3, 1i
bb_bytecode_1:
  %46 = LOAD_FLOAT R2, 8i
  %47 = FLOAT_TO_NUM %46
  STORE_DOUBLE R3, %47
  STORE_TAG R3, tnumber
  INTERRUPT 17u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
