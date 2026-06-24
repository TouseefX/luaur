//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:7588:ir_lowering_uint_source_sanity`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item ir_lowering_uint_source_sanity

#[cfg(test)]
#[test]
fn ir_lowering_uint_source_sanity() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(b: buffer, a: number, s: string)
    local r1 = buffer.readi32(b, bit32.bor(a, 0))
    local r2 = buffer.readu32(b, r1)
    local r3 = buffer.readi32(b, r2)
    local r4 = buffer.readu32(b, string.len(s))
    return r1, r2, r3, r4
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
  CHECK_TAG R1, tnumber, exit(entry)
  CHECK_TAG R2, tstring, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %11 = LOAD_DOUBLE R1
  %12 = NUM_TO_UINT %11
  %24 = LOAD_POINTER R0
  %26 = TRUNCATE_UINT %12
  CHECK_BUFFER_LEN %24, %26, 0i, 4i, undef, bb_exit_9
   ; exit sync: R5, {%12}
  %28 = BUFFER_READI32 %24, %26, tbuffer
  %29 = INT_TO_NUM %28
  STORE_DOUBLE R3, %29
  STORE_TAG R3, tnumber
  CHECK_BUFFER_LEN %24, %28, 0i, 4i, undef, exit(15)
  %42 = BUFFER_READI32 %24, %28, tbuffer
  %43 = UINT_TO_NUM %42
  STORE_DOUBLE R4, %43
  STORE_TAG R4, tnumber
  CHECK_BUFFER_LEN %24, %42, 0i, 4i, undef, exit(22)
  %56 = BUFFER_READI32 %24, %42, tbuffer
  %57 = INT_TO_NUM %56
  STORE_SPLIT_TVALUE R5, tnumber, %57
  %64 = LOAD_POINTER R2
  %65 = STRING_LEN %64
  CHECK_BUFFER_LEN %24, %65, 0i, 4i, undef, bb_exit_10
   ; exit sync: R8, {%65}
  %79 = BUFFER_READI32 %24, %65, tbuffer
  %80 = UINT_TO_NUM %79
  STORE_DOUBLE R6, %80
  STORE_TAG R6, tnumber
  INTERRUPT 38u
  RETURN R3, 4i
"#;

    assert_eq!(actual, expected);
}
