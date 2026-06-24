//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:5371:ir_lowering_buffer_related_indices_positive_advancing_base`
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
//!   - translates_to -> rust_item ir_lowering_buffer_related_indices_positive_advancing_base

#[cfg(test)]
#[test]
fn ir_lowering_buffer_related_indices_positive_advancing_base() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(buf: buffer, pos: number, a: number, b: number, c: number)
    buffer.writei32(buf, pos, a)
    pos += 4
    buffer.writei32(buf, pos, b)
    pos += 4
    buffer.writei32(buf, pos, c)
    pos += 4

    return pos
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1, $arg2, $arg3, $arg4) line 2
bb_0:
  CHECK_TAG R0, tbuffer, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  CHECK_TAG R2, tnumber, exit(entry)
  CHECK_TAG R3, tnumber, exit(entry)
  CHECK_TAG R4, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %19 = LOAD_POINTER R0
  %20 = LOAD_DOUBLE R1
  %21 = NUM_TO_INT %20
  CHECK_BUFFER_LEN %19, %21, 0i, 12i, %20, exit(2)
  %23 = LOAD_DOUBLE R2
  %24 = NUM_TO_UINT %23
  BUFFER_WRITEI32 %19, %21, %24, tbuffer
  %30 = ADD_NUM %20, 4
  %41 = ADD_INT %21, 4i
  %43 = LOAD_DOUBLE R3
  %44 = NUM_TO_UINT %43
  BUFFER_WRITEI32 %19, %41, %44, tbuffer
  %50 = ADD_NUM %30, 4
  %61 = ADD_INT %21, 8i
  %63 = LOAD_DOUBLE R4
  %64 = NUM_TO_UINT %63
  BUFFER_WRITEI32 %19, %61, %64, tbuffer
  %70 = ADD_NUM %50, 4
  STORE_DOUBLE R1, %70
  INTERRUPT 27u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
