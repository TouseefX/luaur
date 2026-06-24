//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:5424:ir_lowering_buffer_related_indices_negative_base`
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
//!   - translates_to -> rust_item ir_lowering_buffer_related_indices_negative_base

#[cfg(test)]
#[test]
fn ir_lowering_buffer_related_indices_negative_base() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(buf: buffer, a: number)
    return buffer.readi32(buf, a - 8) + buffer.readi32(buf, a - 4) + buffer.readi32(buf, a - 0)
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
  %8 = LOAD_DOUBLE R1
  %9 = SUB_NUM %8, 8
  %17 = LOAD_POINTER R0
  %19 = NUM_TO_INT %9
  CHECK_BUFFER_LEN %17, %19, 0i, 12i, %9, bb_exit_6
   ; exit sync: R6, {%9}
  %21 = BUFFER_READI32 %17, %19, tbuffer
  %22 = INT_TO_NUM %21
  %39 = ADD_INT %19, 4i
  %41 = BUFFER_READI32 %17, %39, tbuffer
  %42 = INT_TO_NUM %41
  %52 = ADD_NUM %22, %42
  %68 = ADD_INT %19, 8i
  %70 = BUFFER_READI32 %17, %68, tbuffer
  %71 = INT_TO_NUM %70
  %81 = ADD_NUM %52, %71
  STORE_DOUBLE R2, %81
  STORE_TAG R2, tnumber
  INTERRUPT 23u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
