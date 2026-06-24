//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:7450:ir_lowering_loop_step_detection_1`
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
//!   - type_ref -> enum IncludeRegFlowInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - calls -> method LoweringFixture::getCodegenAssembly (tests/IrLowering.test.cpp)
//!   - translates_to -> rust_item ir_lowering_loop_step_detection_1

#[cfg(test)]
#[test]
fn ir_lowering_loop_step_detection_1() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_code_gen::enums::include_reg_flow_info::IncludeRegFlowInfo;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _load_propagate_origin = ScopedFastFlag::new(&FFlag::LuauCodegenLoadPropagateOrigin, true);

    let mut fixture = LoweringFixture::default();
    fixture.assembly_options.include_reg_flow_info = IncludeRegFlowInfo::Yes;

    let source = CString::new(
        r#"
local function foo(n: number)
    local s = 0
    for i = 1,n do
        s += i
    end
    return s
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
bb_0:
; in regs: R0
; out regs: R0
  CHECK_TAG R0, tnumber, exit(entry)
  JUMP bb_4
bb_4:
; in regs: R0
; out regs: R0
  JUMP bb_bytecode_1
bb_bytecode_1:
; in regs: R0
; out regs: R1, R2, R3, R4
  STORE_DOUBLE R1, 0
  STORE_TAG R1, tnumber
  STORE_DOUBLE R4, 1
  STORE_TAG R4, tnumber
  %8 = LOAD_TVALUE R0, 0i, tnumber
  STORE_TVALUE R2, %8
  STORE_DOUBLE R3, 1
  STORE_TAG R3, tnumber
  %16 = LOAD_DOUBLE R0
  JUMP_CMP_NUM 1, %16, not_le, bb_bytecode_3, bb_bytecode_2
bb_bytecode_2:
; in regs: R1, R2, R3, R4
; out regs: R1, R2, R3, R4
  INTERRUPT 5u
  CHECK_TAG R1, tnumber, exit(5)
  CHECK_TAG R4, tnumber, exit(5)
  %24 = LOAD_DOUBLE R1
  %25 = LOAD_DOUBLE R4
  %26 = ADD_NUM %24, %25
  STORE_DOUBLE R1, %26
  %28 = LOAD_DOUBLE R2
  %30 = ADD_NUM %25, 1
  STORE_DOUBLE R4, %30
  JUMP_CMP_NUM %30, %28, le, bb_bytecode_2, bb_bytecode_3
bb_bytecode_3:
; in regs: R1
  INTERRUPT 7u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
