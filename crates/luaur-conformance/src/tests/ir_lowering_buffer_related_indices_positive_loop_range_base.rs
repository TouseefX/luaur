//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:5286:ir_lowering_buffer_related_indices_positive_loop_range_base`
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
//!   - translates_to -> rust_item ir_lowering_buffer_related_indices_positive_loop_range_base

#[cfg(test)]
#[test]
fn ir_lowering_buffer_related_indices_positive_loop_range_base() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);
    let _call_feedback = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
    let _emit_call_feedback = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(buf: buffer, a: number)
    local s = 0
    for i = 0, buffer.len(buf) - 1, 12 do
        s += buffer.readf32(buf, i) * buffer.readf32(buf, i + 4) * buffer.readf32(buf, i + 8)
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
; function foo($arg0, $arg1) line 2
bb_0:
  CHECK_TAG R0, tbuffer, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  JUMP bb_4
bb_4:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  STORE_DOUBLE R2, 0
  STORE_TAG R2, tnumber
  STORE_DOUBLE R5, 0
  STORE_TAG R5, tnumber
  GET_CACHED_IMPORT R6, K3 (nil), 2148534272u ('buffer'.'len'), 3u
  %12 = LOAD_TVALUE R0, 0i, tbuffer
  STORE_TVALUE R7, %12
  INTERRUPT 5u
  SET_SAVEDPC 7u
  CALL R6, 1i, 1i
  CHECK_TAG R6, tnumber, bb_fallback_5
  %19 = LOAD_DOUBLE R6
  %20 = SUB_NUM %19, 1
  STORE_DOUBLE R3, %20
  STORE_TAG R3, tnumber
  JUMP bb_6
bb_6:
  STORE_DOUBLE R4, 12
  STORE_TAG R4, tnumber
  CHECK_TAG R3, tnumber, exit(9)
  CHECK_TAG R5, tnumber, exit(9)
  %33 = LOAD_DOUBLE R3
  JUMP_CMP_NUM R5, %33, not_le, bb_bytecode_3, bb_bytecode_2
bb_bytecode_2:
  implicit CHECK_SAFE_ENV exit(10)
  INTERRUPT 10u
  CHECK_TAG R5, tnumber, exit(12)
  %42 = LOAD_POINTER R0
  %43 = LOAD_DOUBLE R5
  %44 = NUM_TO_INT %43
  CHECK_BUFFER_LEN %42, %44, 0i, 12i, %43, exit(12)
  %46 = BUFFER_READF32 %42, %44, tbuffer
  %47 = FLOAT_TO_NUM %46
  %64 = ADD_INT %44, 4i
  %66 = BUFFER_READF32 %42, %64, tbuffer
  %67 = FLOAT_TO_NUM %66
  %77 = MUL_NUM %47, %67
  %93 = ADD_INT %44, 8i
  %95 = BUFFER_READF32 %42, %93, tbuffer
  %96 = FLOAT_TO_NUM %95
  %106 = MUL_NUM %77, %96
  CHECK_TAG R2, tnumber, bb_exit_10
   ; exit sync: R8, R7, R6, {%96, %77, %106}
  %113 = LOAD_DOUBLE R2
  %115 = ADD_NUM %113, %106
  STORE_DOUBLE R2, %115
  %117 = LOAD_DOUBLE R3
  %119 = ADD_NUM %43, 12
  STORE_DOUBLE R5, %119
  JUMP_CMP_NUM %119, %117, le, bb_bytecode_2, bb_bytecode_3
bb_bytecode_3:
  INTERRUPT 35u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
