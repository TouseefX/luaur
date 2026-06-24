//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:5567:ir_lowering_buffer_sanity_negative`
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
//!   - calls -> function writeu8 (CodeGen/src/ByteUtils.h)
//!   - calls -> function writeu16 (CodeGen/src/ByteUtils.h)
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item ir_lowering_buffer_sanity_negative

#[cfg(test)]
#[test]
fn ir_lowering_buffer_sanity_negative() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(one: number, b1: buffer, b2: buffer)
    buffer.writei8(b1, one - 1, buffer.readi8(b1, one - 1))
    buffer.writeu8(b1, one - 1, buffer.readu8(b1, one - 1))

    buffer.writei8(b2, one - 1, buffer.readi8(b2, one - 1))
    buffer.writeu8(b2, one - 1, buffer.readu8(b2, one - 1))
    buffer.writei8(b2, one - 0, buffer.readi8(b2, one - 0))
    buffer.writeu8(b2, one - 0, buffer.readu8(b2, one - 0))
    buffer.writei16(b2, one - 1, buffer.readi16(b2, one - 1))
    buffer.writeu16(b2, one - 1, buffer.readu16(b2, one - 1))
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1, $arg2) line 2
bb_0:
  CHECK_TAG R0, tnumber, exit(entry)
  CHECK_TAG R1, tbuffer, exit(entry)
  CHECK_TAG R2, tbuffer, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %10 = LOAD_DOUBLE R0
  %11 = SUB_NUM %10, 1
  %25 = LOAD_POINTER R1
  %27 = NUM_TO_INT %11
  CHECK_BUFFER_LEN %25, %27, 0i, 1i, undef, bb_exit_19
   ; exit sync: R8, R5, {%11}
  %29 = BUFFER_READI8 %25, %27, tbuffer
  BUFFER_WRITEI8 %25, %27, %29, tbuffer
  %70 = BUFFER_READU8 %25, %27, tbuffer
  BUFFER_WRITEI8 %25, %27, %70, tbuffer
  %107 = LOAD_POINTER R2
  CHECK_BUFFER_LEN %107, %27, 0i, 2i, %11, exit(32)
  %111 = BUFFER_READI8 %107, %27, tbuffer
  BUFFER_WRITEI8 %107, %27, %111, tbuffer
  %152 = BUFFER_READU8 %107, %27, tbuffer
  BUFFER_WRITEI8 %107, %27, %152, tbuffer
  %191 = ADD_INT %27, 1i
  %193 = BUFFER_READI8 %107, %191, tbuffer
  BUFFER_WRITEI8 %107, %191, %193, tbuffer
  %234 = BUFFER_READU8 %107, %191, tbuffer
  BUFFER_WRITEI8 %107, %191, %234, tbuffer
  %275 = BUFFER_READI16 %107, %27, tbuffer
  BUFFER_WRITEI16 %107, %27, %275, tbuffer
  %316 = BUFFER_READU16 %107, %27, tbuffer
  BUFFER_WRITEI16 %107, %27, %316, tbuffer
  INTERRUPT 112u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
