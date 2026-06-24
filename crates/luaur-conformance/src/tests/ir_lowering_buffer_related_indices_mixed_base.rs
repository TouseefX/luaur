//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:5468:ir_lowering_buffer_related_indices_mixed_base`
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
//!   - translates_to -> rust_item ir_lowering_buffer_related_indices_mixed_base

#[cfg(test)]
#[test]
fn ir_lowering_buffer_related_indices_mixed_base() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(buf: buffer, a: number)
    return buffer.readi32(buf, a) + buffer.readi32(buf, a - 4) + buffer.readi32(buf, a + 4)
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
  %11 = LOAD_POINTER R0
  %12 = LOAD_DOUBLE R1
  %13 = NUM_TO_INT %12
  CHECK_BUFFER_LEN %11, %13, -4i, 8i, %12, exit(2)
  %15 = BUFFER_READI32 %11, %13, tbuffer
  %16 = INT_TO_NUM %15
  %33 = ADD_INT %13, -4i
  %35 = BUFFER_READI32 %11, %33, tbuffer
  %36 = INT_TO_NUM %35
  %46 = ADD_NUM %16, %36
  %62 = ADD_INT %13, 4i
  %64 = BUFFER_READI32 %11, %62, tbuffer
  %65 = INT_TO_NUM %64
  %75 = ADD_NUM %46, %65
  STORE_DOUBLE R2, %75
  STORE_TAG R2, tnumber
  INTERRUPT 23u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
