//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:5840:ir_lowering_buffer_vm_exit_sync`
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
//!   - translates_to -> rust_item ir_lowering_buffer_vm_exit_sync

#[cfg(test)]
#[test]
fn ir_lowering_buffer_vm_exit_sync() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(buf: buffer, a: number, b: number, c: number)
    local x = buffer.readu8(buf, a * b)
    local y = buffer.readu8(buf, a * b + c)
    return x, y
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1, $arg2, $arg3) line 2
bb_0:
  CHECK_TAG R0, tbuffer, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  CHECK_TAG R2, tnumber, exit(entry)
  CHECK_TAG R3, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %14 = LOAD_DOUBLE R1
  %16 = MUL_NUM %14, R2
  %24 = LOAD_POINTER R0
  %26 = NUM_TO_INT %16
  CHECK_BUFFER_LEN %24, %26, 0i, 1i, undef, bb_exit_5
   ; exit sync: R6, {%16}
  %28 = BUFFER_READU8 %24, %26, tbuffer
  %29 = INT_TO_NUM %28
  STORE_DOUBLE R4, %29
  STORE_TAG R4, tnumber
  %48 = ADD_NUM %16, R3
  %58 = NUM_TO_INT %48
  CHECK_BUFFER_LEN %24, %58, 0i, 1i, undef, bb_exit_6
   ; exit sync: R8, R7, {%16, %48}
  %60 = BUFFER_READU8 %24, %58, tbuffer
  %61 = INT_TO_NUM %60
  STORE_DOUBLE R5, %61
  STORE_TAG R5, tnumber
  INTERRUPT 15u
  RETURN R4, 2i
"#;

    assert_eq!(actual, expected);
}
