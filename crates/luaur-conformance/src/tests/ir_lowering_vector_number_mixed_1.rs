//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1673:ir_lowering_vector_number_mixed_1`
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
//!   - translates_to -> rust_item ir_lowering_vector_number_mixed_1

#[cfg(test)]
#[test]
fn ir_lowering_vector_number_mixed_1() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(vectors: {vector}, i)
    local t = i / 100
    return vectors[i] * (1 - t)
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, true)
    );
    let expected = r#"
; function foo($arg0, $arg1) line 2
bb_0:
  CHECK_TAG R0, ttable, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  CHECK_TAG R1, tnumber, bb_fallback_3
  %6 = LOAD_DOUBLE R1
  %7 = DIV_NUM %6, 100
  STORE_DOUBLE R2, %7
  STORE_TAG R2, tnumber
  JUMP bb_linear_11
bb_linear_11:
  %60 = LOAD_POINTER R0
  %62 = TRY_NUM_TO_INDEX %6, bb_fallback_5
  %63 = SUB_INT %62, 1i
  CHECK_ARRAY_SIZE %60, %63, bb_fallback_5
  CHECK_NO_METATABLE %60, bb_fallback_5
  %66 = GET_ARR_ADDR %60, %63
  %67 = LOAD_TVALUE %66
  STORE_TVALUE R4, %67
  %73 = SUB_NUM 1, %7
  CHECK_TAG R4, tvector, bb_exit_12
   ; exit sync: R5, {%73}
  %83 = NUM_TO_FLOAT %73
  %84 = FLOAT_TO_VEC %83
  %85 = MUL_VEC %67, %84
  %86 = TAG_VECTOR %85
  STORE_TVALUE R3, %86
  INTERRUPT 4u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
