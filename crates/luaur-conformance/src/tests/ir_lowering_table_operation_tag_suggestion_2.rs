//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:7836:ir_lowering_table_operation_tag_suggestion_2`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item ir_lowering_table_operation_tag_suggestion_2

#[cfg(test)]
#[test]
fn ir_lowering_table_operation_tag_suggestion_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
    let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(self, t: { id: string }, a: number)
    self.map[t.id] = self.map[t.id] + a
    self.foo(self.map[t.id])
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, true)
    );
    let expected = r#"
; function test($arg0, $arg1, $arg2) line 2
; R1: table [argument]
; R2: number [argument]
; R4: string from 10 to 11
; R6: string from 17 to 18
; R8: string from 8 to 9
bb_0:
  CHECK_TAG R1, ttable, exit(entry)
  CHECK_TAG R2, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  CHECK_TAG R0, ttable, bb_fallback_3
  %8 = LOAD_POINTER R0
  %9 = GET_SLOT_NODE_ADDR %8, 0u, K0 ('map')
  CHECK_SLOT_MATCH %9, K0 ('map'), bb_fallback_3
  %11 = LOAD_TVALUE %9, 0i
  STORE_TVALUE R3, %11
  JUMP bb_linear_19
bb_linear_19:
  %100 = LOAD_POINTER R1
  %101 = GET_SLOT_NODE_ADDR %100, 2u, K1 ('id')
  CHECK_SLOT_MATCH %101, K1 ('id'), bb_fallback_5
  %103 = LOAD_TVALUE %101, 0i
  STORE_TVALUE R4, %103
  STORE_TVALUE R7, %11
  STORE_TVALUE R8, %103
  SET_SAVEDPC 9u
  GET_TABLE R6, R7, R8
  CHECK_TAG R6, tnumber, bb_fallback_11
  %124 = LOAD_DOUBLE R6
  %126 = ADD_NUM %124, R2
  STORE_DOUBLE R5, %126
  STORE_TAG R5, tnumber
  SET_SAVEDPC 11u
  SET_TABLE R5, R3, R4
  %134 = LOAD_POINTER R0
  %135 = GET_SLOT_NODE_ADDR %134, 11u, K2 ('foo')
  CHECK_SLOT_MATCH %135, K2 ('foo'), bb_fallback_13
  %137 = LOAD_TVALUE %135, 0i
  STORE_TVALUE R3, %137
  %143 = GET_SLOT_NODE_ADDR %134, 13u, K0 ('map')
  CHECK_SLOT_MATCH %143, K0 ('map'), bb_fallback_15
  %145 = LOAD_TVALUE %143, 0i
  STORE_TVALUE R5, %145
  %148 = LOAD_POINTER R1
  %149 = GET_SLOT_NODE_ADDR %148, 15u, K1 ('id')
  CHECK_SLOT_MATCH %149, K1 ('id'), bb_fallback_17
  %151 = LOAD_TVALUE %149, 0i
  STORE_TVALUE R6, %151
  SET_SAVEDPC 18u
  GET_TABLE R4, R5, R6
  INTERRUPT 18u
  SET_SAVEDPC 20u
  CALL R3, 1i, 0i
  INTERRUPT 20u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
