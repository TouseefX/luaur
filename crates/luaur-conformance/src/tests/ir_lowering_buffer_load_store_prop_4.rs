//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:7338:ir_lowering_buffer_load_store_prop_4`
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
//!   - calls -> function writef64 (CodeGen/src/ByteUtils.h)
//!   - calls -> function writeu8 (CodeGen/src/ByteUtils.h)
//!   - calls -> function writeu16 (CodeGen/src/ByteUtils.h)
//!   - calls -> function writeu32 (CodeGen/src/ByteUtils.h)
//!   - calls -> function writef32 (CodeGen/src/ByteUtils.h)
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item ir_lowering_buffer_load_store_prop_4

#[cfg(test)]
#[test]
fn ir_lowering_buffer_load_store_prop_4() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(b: buffer, n: number, f: number)
    buffer.writei8(b, 0, n)
    buffer.writef64(b, 100, buffer.readi8(b, 0))
    buffer.writei8(b, 108, buffer.readi8(b, 0))
    buffer.writeu8(b, 109, buffer.readi8(b, 0))

    buffer.writeu8(b, 2, n)
    buffer.writef64(b, 116, buffer.readu8(b, 2))
    buffer.writeu8(b, 124, buffer.readu8(b, 2))
    buffer.writei8(b, 125, buffer.readu8(b, 2))

    buffer.writei16(b, 4, n)
    buffer.writef64(b, 132, buffer.readi16(b, 4))
    buffer.writei16(b, 140, buffer.readi16(b, 4))
    buffer.writeu16(b, 142, buffer.readi16(b, 4))

    buffer.writeu16(b, 8, n)
    buffer.writef64(b, 148, buffer.readu16(b, 8))
    buffer.writeu16(b, 156, buffer.readu16(b, 8))
    buffer.writei16(b, 158, buffer.readu16(b, 8))

    buffer.writei32(b, 12, n)
    buffer.writef64(b, 164, buffer.readi32(b, 12))
    buffer.writei32(b, 172, buffer.readi32(b, 12))
    buffer.writeu32(b, 176, buffer.readi32(b, 12))

    buffer.writeu32(b, 20, n)
    buffer.writef64(b, 180, buffer.readu32(b, 20))
    buffer.writeu32(b, 188, buffer.readu32(b, 20))
    buffer.writei32(b, 192, buffer.readu32(b, 20))

    buffer.writef32(b, 28, f)
    buffer.writef64(b, 196, buffer.readf32(b, 28))
    buffer.writef32(b, 196, buffer.readf32(b, 28))

    buffer.writef64(b, 32, f)
    buffer.writef64(b, 204, buffer.readf64(b, 32))
    buffer.writef32(b, 204, buffer.readf64(b, 32))
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function test($arg0, $arg1, $arg2) line 2
bb_0:
  CHECK_TAG R0, tbuffer, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  CHECK_TAG R2, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %17 = LOAD_POINTER R0
  CHECK_BUFFER_LEN %17, 0i, 0i, 212i, undef, bb_exit_55
   ; exit sync: R5, {}
  %21 = LOAD_DOUBLE R1
  %22 = NUM_TO_UINT %21
  BUFFER_WRITEI8 %17, 0i, %22, tbuffer
  %33 = SEXTI8_INT %22
  %34 = INT_TO_NUM %33
  BUFFER_WRITEF64 %17, 100i, %34, tbuffer
  BUFFER_WRITEI8 %17, 108i, %22, tbuffer
  BUFFER_WRITEI8 %17, 109i, %22, tbuffer
  BUFFER_WRITEI8 %17, 2i, %22, tbuffer
  %133 = BITAND_UINT %22, 255i
  %134 = INT_TO_NUM %133
  BUFFER_WRITEF64 %17, 116i, %134, tbuffer
  BUFFER_WRITEI8 %17, 124i, %22, tbuffer
  BUFFER_WRITEI8 %17, 125i, %22, tbuffer
  BUFFER_WRITEI16 %17, 4i, %22, tbuffer
  %233 = SEXTI16_INT %22
  %234 = INT_TO_NUM %233
  BUFFER_WRITEF64 %17, 132i, %234, tbuffer
  BUFFER_WRITEI16 %17, 140i, %22, tbuffer
  BUFFER_WRITEI16 %17, 142i, %22, tbuffer
  BUFFER_WRITEI16 %17, 8i, %22, tbuffer
  %333 = BITAND_UINT %22, 65535i
  %334 = INT_TO_NUM %333
  BUFFER_WRITEF64 %17, 148i, %334, tbuffer
  BUFFER_WRITEI16 %17, 156i, %22, tbuffer
  BUFFER_WRITEI16 %17, 158i, %22, tbuffer
  BUFFER_WRITEI32 %17, 12i, %22, tbuffer
  %433 = TRUNCATE_UINT %22
  %434 = INT_TO_NUM %433
  BUFFER_WRITEF64 %17, 164i, %434, tbuffer
  BUFFER_WRITEI32 %17, 172i, %22, tbuffer
  BUFFER_WRITEI32 %17, 176i, %22, tbuffer
  BUFFER_WRITEI32 %17, 20i, %22, tbuffer
  %534 = UINT_TO_NUM %22
  BUFFER_WRITEF64 %17, 180i, %534, tbuffer
  BUFFER_WRITEI32 %17, 188i, %22, tbuffer
  BUFFER_WRITEI32 %17, 192i, %22, tbuffer
  %621 = LOAD_DOUBLE R2
  %622 = NUM_TO_FLOAT %621
  BUFFER_WRITEF32 %17, 28i, %622, tbuffer
  %634 = FLOAT_TO_NUM %622
  BUFFER_WRITEF64 %17, 196i, %634, tbuffer
  BUFFER_WRITEF32 %17, 196i, %622, tbuffer
  BUFFER_WRITEF64 %17, 32i, %621, tbuffer
  BUFFER_WRITEF64 %17, 204i, %621, tbuffer
  BUFFER_WRITEF32 %17, 204i, %622, tbuffer
  INTERRUPT 372u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
