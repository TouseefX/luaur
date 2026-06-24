//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:7185:ir_lowering_buffer_load_store_prop_2`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item ir_lowering_buffer_load_store_prop_2

#[cfg(test)]
#[test]
fn ir_lowering_buffer_load_store_prop_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(b: buffer)
    buffer.writei8(b, 10, 32)
    assert(buffer.readi8(b, 10) == 32)

    buffer.writei8(b, 14, 4)
    buffer.writei8(b, 13, 3)
    buffer.writei8(b, 12, 2)
    buffer.writei8(b, 11, 1)

    return buffer.readi8(b, 11) + buffer.readi8(b, 12) + buffer.readi8(b, 14) + buffer.readi8(b, 13)
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function test($arg0) line 2
bb_0:
  CHECK_TAG R0, tbuffer, exit(entry)
  JUMP bb_4
bb_4:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %15 = LOAD_POINTER R0
  CHECK_BUFFER_LEN %15, 10i, 0i, 5i, undef, bb_exit_17
   ; exit sync: R4, R3, {}
  BUFFER_WRITEI8 %15, 10i, 32i, tbuffer
  JUMP bb_bytecode_3
bb_bytecode_3:
  JUMP bb_8
bb_8:
  BUFFER_WRITEI8 %15, 14i, 4i, tbuffer
  BUFFER_WRITEI8 %15, 13i, 3i, tbuffer
  BUFFER_WRITEI8 %15, 12i, 2i, tbuffer
  BUFFER_WRITEI8 %15, 11i, 1i, tbuffer
  STORE_DOUBLE R1, 10
  STORE_TAG R1, tnumber
  INTERRUPT 86u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
