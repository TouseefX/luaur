//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:5757:ir_lowering_buffer_related_indices_positive_mult_base_int`
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
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item ir_lowering_buffer_related_indices_positive_mult_base_int

#[cfg(test)]
#[test]
fn ir_lowering_buffer_related_indices_positive_mult_base_int() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(buf: buffer, a: number)
    -- trying to be helpful
    local t1 = bit32.bor(a, 0)
    local t2 = bit32.bor(t1 + 8, 0)
    local t3 = bit32.bor(t1 + 16, 0)
    return buffer.readf64(buf, t1) + buffer.readf64(buf, t2) + buffer.readf64(buf, t3)
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
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %9 = LOAD_DOUBLE R1
  %10 = NUM_TO_UINT %9
  %27 = ADD_INT %10, 8i
  %44 = ADD_INT %10, 16i
  %56 = LOAD_POINTER R0
  %58 = TRUNCATE_UINT %10
  CHECK_BUFFER_LEN %56, %58, 0i, 24i, undef, bb_exit_9
   ; exit sync: R4, R3, R2, {%44, %27, %10}
  %60 = BUFFER_READF64 %56, %58, tbuffer
  %73 = BUFFER_READF64 %56, %27, tbuffer
  %83 = ADD_NUM %60, %73
  %95 = BUFFER_READF64 %56, %44, tbuffer
  %105 = ADD_NUM %83, %95
  STORE_SPLIT_TVALUE R5, tnumber, %105
  INTERRUPT 44u
  RETURN R5, 1i
"#;

    assert_eq!(actual, expected);
}
