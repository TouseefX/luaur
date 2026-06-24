//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:5888:ir_lowering_buffer_effects`
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
//!   - calls -> function writef64 (CodeGen/src/ByteUtils.h)
//!   - calls -> function writeu8 (CodeGen/src/ByteUtils.h)
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item ir_lowering_buffer_effects

#[cfg(test)]
#[test]
fn ir_lowering_buffer_effects() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(buf: buffer)
    buffer.writef64(buf, 0, 3.14)
    local u1 = buffer.writeu8(buf, 4, 170)
    local u2 = buffer.writeu8(buf, 5, 187)
    local u3 = buffer.writeu8(buf, 0, 255)
    return buffer.readf64(buf, 0), u1, u2, u3
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, true)
    );
    let expected = r#"
; function foo($arg0) line 2
bb_0:
  CHECK_TAG R0, tbuffer, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %15 = LOAD_POINTER R0
  CHECK_BUFFER_LEN %15, 0i, 0i, 8i, undef, bb_exit_12
   ; exit sync: R4, R3, {}
  BUFFER_WRITEF64 %15, 0i, 3.1400000000000001, tbuffer
  STORE_SPLIT_TVALUE R3, tnumber, 4
  STORE_SPLIT_TVALUE R4, tnumber, 170
  SET_SAVEDPC 12u
  %28 = INVOKE_FASTCALL 67u, R1, R0, R3, R4, 3i, 1i
  CHECK_FASTCALL_RES %28, bb_fallback_4
  JUMP bb_linear_11
bb_linear_11:
  STORE_DOUBLE R4, 5
  STORE_TAG R4, tnumber
  STORE_DOUBLE R5, 187
  STORE_TAG R5, tnumber
  SET_SAVEDPC 20u
  %97 = INVOKE_FASTCALL 67u, R2, R0, R4, R5, 3i, 1i
  CHECK_FASTCALL_RES %97, bb_fallback_6
  STORE_DOUBLE R5, 0
  STORE_TAG R5, tnumber
  STORE_DOUBLE R6, 255
  STORE_TAG R6, tnumber
  SET_SAVEDPC 28u
  %106 = INVOKE_FASTCALL 67u, R3, R0, R5, R6, 3i, 1i
  CHECK_FASTCALL_RES %106, bb_fallback_8
  CHECK_BUFFER_LEN %15, 0i, 0i, 8i, undef, exit(34)
  %112 = BUFFER_READF64 %15, 0i, tbuffer
  STORE_DOUBLE R4, %112
  STORE_TAG R4, tnumber
  %115 = LOAD_TVALUE R1
  STORE_TVALUE R5, %115
  %117 = LOAD_TVALUE R2
  STORE_TVALUE R6, %117
  %119 = LOAD_TVALUE R3
  STORE_TVALUE R7, %119
  INTERRUPT 42u
  RETURN R4, 4i
"#;

    assert_eq!(actual, expected);
}
