//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:7019:ir_lowering_upvalue_access_load_store_4`
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
//!   - translates_to -> rust_item ir_lowering_upvalue_access_load_store_4

#[cfg(test)]
#[test]
fn ir_lowering_upvalue_access_load_store_4() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _extra_table_opts = ScopedFastFlag::new(&FFlag::LuauCodegenExtraTableOpts, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local arr: {number}

local function foo(a: number)
    for i = 1,#arr do
        arr[i] = arr[i] + arr[i] * a
    end
end

arr = {1, 2, 3, 4}
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0) line 4
bb_0:
  CHECK_TAG R0, tnumber, exit(entry)
  JUMP bb_4
bb_4:
  JUMP bb_bytecode_1
bb_bytecode_1:
  STORE_DOUBLE R3, 1
  STORE_TAG R3, tnumber
  %6 = GET_UPVALUE U0
  STORE_TVALUE R4, %6
  CHECK_TAG R4, ttable, exit(2)
  %10 = LOAD_POINTER R4
  CHECK_NO_METATABLE %10, bb_fallback_5
  %12 = TABLE_LEN %10
  %13 = INT_TO_NUM %12
  STORE_DOUBLE R1, %13
  STORE_TAG R1, tnumber
  JUMP bb_6
bb_6:
  STORE_DOUBLE R2, 1
  STORE_TAG R2, tnumber
  CHECK_TAG R1, tnumber, exit(4)
  CHECK_TAG R3, tnumber, exit(4)
  %26 = LOAD_DOUBLE R1
  JUMP_CMP_NUM R3, %26, not_le, bb_bytecode_3, bb_bytecode_2
bb_bytecode_2:
  INTERRUPT 5u
  %30 = GET_UPVALUE U0
  STORE_TVALUE R4, %30
  STORE_TVALUE R7, %30
  CHECK_TAG R7, ttable, exit(7)
  CHECK_TAG R3, tnumber, exit(7)
  %38 = LOAD_POINTER R7
  %39 = LOAD_DOUBLE R3
  %40 = TRY_NUM_TO_INDEX %39, bb_fallback_7
  %41 = SUB_INT %40, 1i
  CHECK_ARRAY_SIZE %38, %41, bb_fallback_7
  CHECK_NO_METATABLE %38, bb_fallback_7
  %44 = GET_ARR_ADDR %38, %41
  %45 = LOAD_TVALUE %44
  STORE_TVALUE R6, %45
  JUMP bb_linear_17
bb_linear_17:
  STORE_TVALUE R8, %45
  CHECK_TAG R8, tnumber, bb_fallback_11
  %141 = LOAD_DOUBLE R8
  %143 = MUL_NUM %141, R0
  %153 = ADD_NUM %141, %143
  STORE_DOUBLE R5, %153
  STORE_TAG R5, tnumber
  CHECK_READONLY %38, bb_fallback_15
  STORE_SPLIT_TVALUE %44, tnumber, %153
  %173 = LOAD_DOUBLE R1
  %175 = ADD_NUM %39, 1
  STORE_DOUBLE R3, %175
  JUMP_CMP_NUM %175, %173, le, bb_bytecode_2, bb_bytecode_3
bb_8:
  %51 = GET_UPVALUE U0
  STORE_TVALUE R9, %51
  CHECK_TAG R9, ttable, exit(9)
  CHECK_TAG R3, tnumber, exit(9)
  %57 = LOAD_POINTER R9
  %58 = LOAD_DOUBLE R3
  %59 = TRY_NUM_TO_INDEX %58, bb_fallback_9
  %60 = SUB_INT %59, 1i
  CHECK_ARRAY_SIZE %57, %60, bb_fallback_9
  CHECK_NO_METATABLE %57, bb_fallback_9
  %63 = GET_ARR_ADDR %57, %60
  %64 = LOAD_TVALUE %63
  STORE_TVALUE R8, %64
  JUMP bb_10
bb_10:
  CHECK_TAG R8, tnumber, bb_fallback_11
  %74 = LOAD_DOUBLE R8
  %76 = MUL_NUM %74, R0
  STORE_DOUBLE R7, %76
  STORE_TAG R7, tnumber
  JUMP bb_12
bb_12:
  CHECK_TAG R6, tnumber, bb_fallback_13
  CHECK_TAG R7, tnumber, bb_fallback_13
  %87 = LOAD_DOUBLE R6
  %89 = ADD_NUM %87, R7
  STORE_DOUBLE R5, %89
  STORE_TAG R5, tnumber
  JUMP bb_14
bb_14:
  CHECK_TAG R4, ttable, exit(12)
  CHECK_TAG R3, tnumber, exit(12)
  %100 = LOAD_POINTER R4
  %101 = LOAD_DOUBLE R3
  %102 = TRY_NUM_TO_INDEX %101, bb_fallback_15
  %103 = SUB_INT %102, 1i
  CHECK_ARRAY_SIZE %100, %103, bb_fallback_15
  CHECK_NO_METATABLE %100, bb_fallback_15
  CHECK_READONLY %100, bb_fallback_15
  %107 = GET_ARR_ADDR %100, %103
  %108 = LOAD_TVALUE R5
  STORE_TVALUE %107, %108
  BARRIER_TABLE_FORWARD %100, R5, undef
  JUMP bb_16
bb_16:
  %115 = LOAD_DOUBLE R1
  %116 = LOAD_DOUBLE R3
  %117 = ADD_NUM %116, 1
  STORE_DOUBLE R3, %117
  JUMP_CMP_NUM %117, %115, le, bb_bytecode_2, bb_bytecode_3
bb_bytecode_3:
  INTERRUPT 14u
  RETURN R0, 0i
"#;

    assert_eq!(actual, expected);
}
