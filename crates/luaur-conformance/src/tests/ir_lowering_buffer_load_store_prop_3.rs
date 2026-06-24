//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:7233:ir_lowering_buffer_load_store_prop_3`
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
//!   - calls -> function writeu32 (CodeGen/src/ByteUtils.h)
//!   - calls -> function writeu16 (CodeGen/src/ByteUtils.h)
//!   - calls -> function writeu8 (CodeGen/src/ByteUtils.h)
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item ir_lowering_buffer_load_store_prop_3

#[cfg(test)]
#[test]
fn ir_lowering_buffer_load_store_prop_3() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function storeloadpreserve(b: buffer)
    buffer.writeu32(b, 0, 0xffffffff)
    assert(buffer.readi32(b, 0) == -1)
    assert(buffer.readu32(b, 0) == 4294967295)

    buffer.writei32(b, 0, -1)
    assert(buffer.readi32(b, 0) == -1)
    assert(buffer.readu32(b, 0) == 4294967295)

    buffer.writei16(b, 0, 65535)
    assert(buffer.readi16(b, 0) == -1)
    assert(buffer.readu16(b, 0) == 65535)

    buffer.writeu16(b, 0, 65535)
    assert(buffer.readi16(b, 0) == -1)
    assert(buffer.readu16(b, 0) == 65535)

    buffer.writeu8(b, 0, 0xffffffff)
    assert(buffer.readi8(b, 0) == -1)
    assert(buffer.readu8(b, 0) == 255)

    buffer.writeu16(b, 0, 0xffffffff)
    assert(buffer.readi16(b, 0) == -1)
    assert(buffer.readu16(b, 0) == 65535)
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function storeloadpreserve($arg0) line 2
bb_0:
  CHECK_TAG R0, tbuffer, exit(entry)
  JUMP bb_26
bb_26:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %15 = LOAD_POINTER R0
  CHECK_BUFFER_LEN %15, 0i, 0i, 4i, undef, bb_exit_69
   ; exit sync: R4, R3, {}
  BUFFER_WRITEI32 %15, 0i, -1i, tbuffer
  JUMP bb_bytecode_3
bb_bytecode_3:
  JUMP bb_30
bb_30:
  JUMP bb_bytecode_5
bb_bytecode_5:
  JUMP bb_33
bb_33:
  BUFFER_WRITEI32 %15, 0i, -1i, tbuffer
  JUMP bb_bytecode_7
bb_bytecode_7:
  JUMP bb_37
bb_37:
  JUMP bb_bytecode_9
bb_bytecode_9:
  JUMP bb_40
bb_40:
  BUFFER_WRITEI16 %15, 0i, 65535i, tbuffer
  JUMP bb_bytecode_11
bb_bytecode_11:
  JUMP bb_44
bb_44:
  JUMP bb_bytecode_13
bb_bytecode_13:
  JUMP bb_47
bb_47:
  BUFFER_WRITEI16 %15, 0i, 65535i, tbuffer
  JUMP bb_bytecode_15
bb_bytecode_15:
  JUMP bb_51
bb_51:
  JUMP bb_bytecode_17
bb_bytecode_17:
  JUMP bb_54
bb_54:
  BUFFER_WRITEI8 %15, 0i, -1i, tbuffer
  JUMP bb_bytecode_19
bb_bytecode_19:
  JUMP bb_58
bb_58:
  JUMP bb_bytecode_21
bb_bytecode_21:
  JUMP bb_61
bb_61:
  BUFFER_WRITEI16 %15, 0i, -1i, tbuffer
  JUMP bb_bytecode_23
bb_bytecode_23:
  JUMP bb_65
bb_65:
  JUMP bb_bytecode_25
bb_bytecode_25:
  JUMP bb_68
bb_68:
  INTERRUPT 228u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
