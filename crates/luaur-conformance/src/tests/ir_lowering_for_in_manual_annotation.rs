//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3826:ir_lowering_for_in_manual_annotation`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Vertex (tests/ConformanceIrHooks.h)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item ir_lowering_for_in_manual_annotation

#[cfg(test)]
#[test]
fn ir_lowering_for_in_manual_annotation() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
    let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
type Vertex = {pos: vector, normal: vector}

local function foo(a: {Vertex})
    local sum = 0
    for k, v: Vertex in ipairs(a) do
        sum += v.pos.X
    end
    return sum
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 2, 2, false)
    );
    let expected = r#"
; function foo(a) line 4
; R0: table [argument 'a']
; R1: number from 0 to 15 [local 'sum']
; R5: number from 6 to 12 [local 'k']
; R6: table from 6 to 12 [local 'v']
; R7: vector from 9 to 11
bb_0:
  CHECK_TAG R0, ttable, exit(entry)
  JUMP bb_4
bb_4:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  STORE_DOUBLE R1, 0
  STORE_TAG R1, tnumber
  GET_CACHED_IMPORT R2, K1 (nil), 1073741824u ('ipairs'), 2u
  %8 = LOAD_TVALUE R0, 0i, ttable
  STORE_TVALUE R3, %8
  INTERRUPT 4u
  SET_SAVEDPC 6u
  CALL R2, 1i, 3i
  CHECK_SAFE_ENV exit(6)
  CHECK_TAG R3, ttable, bb_fallback_5
  CHECK_TAG R4, tnumber, bb_fallback_5
  JUMP_CMP_NUM R4, 0, not_eq, bb_fallback_5, bb_6
bb_6:
  STORE_TAG R2, tnil
  STORE_POINTER R4, 0i
  STORE_EXTRA R4, 128i
  STORE_TAG R4, tlightuserdata
  JUMP bb_bytecode_3
bb_bytecode_2:
  CHECK_TAG R6, ttable, exit(7)
  %28 = LOAD_POINTER R6
  %29 = GET_SLOT_NODE_ADDR %28, 7u, K2 ('pos')
  CHECK_SLOT_MATCH %29, K2 ('pos'), bb_fallback_7
  %31 = LOAD_TVALUE %29, 0i
  STORE_TVALUE R7, %31
  JUMP bb_8
bb_8:
  CHECK_TAG R7, tvector, exit(9)
  %38 = LOAD_FLOAT R7, 0i
  %39 = FLOAT_TO_NUM %38
  STORE_DOUBLE R7, %39
  STORE_TAG R7, tnumber
  CHECK_TAG R1, tnumber, exit(11)
  %46 = LOAD_DOUBLE R1
  %48 = ADD_NUM %46, %39
  STORE_DOUBLE R1, %48
  JUMP bb_bytecode_3
bb_bytecode_3:
  INTERRUPT 12u
  CHECK_TAG R2, tnil, bb_fallback_10
  %54 = LOAD_POINTER R3
  %55 = LOAD_INT R4
  %56 = GET_ARR_ADDR %54, %55
  CHECK_ARRAY_SIZE %54, %55, bb_9
  %58 = LOAD_TAG %56
  JUMP_EQ_TAG %58, tnil, bb_9, bb_11
bb_11:
  %60 = ADD_INT %55, 1i
  STORE_INT R4, %60
  %62 = INT_TO_NUM %60
  STORE_DOUBLE R5, %62
  STORE_TAG R5, tnumber
  %65 = LOAD_TVALUE %56
  STORE_TVALUE R6, %65
  JUMP bb_bytecode_2
bb_9:
  INTERRUPT 14u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
