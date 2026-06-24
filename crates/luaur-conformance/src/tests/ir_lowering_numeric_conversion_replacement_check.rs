//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:5627:ir_lowering_numeric_conversion_replacement_check`
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
//!   - translates_to -> rust_item ir_lowering_numeric_conversion_replacement_check

#[cfg(test)]
#[test]
fn ir_lowering_numeric_conversion_replacement_check() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(buf: buffer, a: number)
    math.ldexp(a, a) -- generate NUM_TO_INT early

    -- range checks cannot make NUM_TO_INT exit to VM at a later location
    return buffer.readi32(buf, a) + buffer.readi32(buf, a + 4)
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
  %11 = LOAD_DOUBLE R1
  %13 = NUM_TO_INT %11
  %23 = LOAD_POINTER R0
  CHECK_BUFFER_LEN %23, %13, 0i, 8i, %11, bb_exit_6
   ; exit sync: R2, {%11, %13}
  %27 = BUFFER_READI32 %23, %13, tbuffer
  %28 = INT_TO_NUM %27
  %45 = ADD_INT %13, 4i
  %47 = BUFFER_READI32 %23, %45, tbuffer
  %48 = INT_TO_NUM %47
  %58 = ADD_NUM %28, %48
  STORE_SPLIT_TVALUE R2, tnumber, %58
  INTERRUPT 22u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
