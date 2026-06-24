//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:5236:ir_lowering_buffer_related_indices_positive_dynamic_base`
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
//!   - translates_to -> rust_item ir_lowering_buffer_related_indices_positive_dynamic_base

#[cfg(test)]
#[test]
fn ir_lowering_buffer_related_indices_positive_dynamic_base() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(index: buffer, data: buffer, a: number)
    local i = buffer.readi32(index, a)
    return buffer.readf32(data, i + 0) * buffer.readf32(data, i + 4) * buffer.readf32(data, i + 8)
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
  CHECK_TAG R0, tbuffer, exit(entry)
  CHECK_TAG R1, tbuffer, exit(entry)
  CHECK_TAG R2, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %13 = LOAD_POINTER R0
  %14 = LOAD_DOUBLE R2
  %15 = NUM_TO_INT %14
  CHECK_BUFFER_LEN %13, %15, 0i, 4i, undef, exit(2)
  %17 = BUFFER_READI32 %13, %15, tbuffer
  %18 = INT_TO_NUM %17
  %33 = LOAD_POINTER R1
  %35 = NUM_TO_INT %18
  CHECK_BUFFER_LEN %33, %35, 0i, 12i, %18, bb_exit_7
   ; exit sync: R8, R3, {%18}
  %37 = BUFFER_READF32 %33, %35, tbuffer
  %38 = FLOAT_TO_NUM %37
  %55 = ADD_INT %35, 4i
  %57 = BUFFER_READF32 %33, %55, tbuffer
  %58 = FLOAT_TO_NUM %57
  %68 = MUL_NUM %38, %58
  %84 = ADD_INT %35, 8i
  %86 = BUFFER_READF32 %33, %84, tbuffer
  %87 = FLOAT_TO_NUM %86
  %97 = MUL_NUM %68, %87
  STORE_DOUBLE R4, %97
  STORE_TAG R4, tnumber
  INTERRUPT 30u
  RETURN R4, 1i
"#;

    assert_eq!(actual, expected);
}
