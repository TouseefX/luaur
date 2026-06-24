//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3698:ir_lowering_resolve_vector_namecalls`
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
//!   - translates_to -> rust_item ir_lowering_resolve_vector_namecalls

#[cfg(test)]
#[test]
fn ir_lowering_resolve_vector_namecalls() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
type Vertex = {pos: vector, normal: vector}

local function foo(arr: {Vertex}, i)
    return arr[i].normal:Dot(vector(0.707, 0, 0.707))
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1) line 4
; R0: table [argument]
; R2: vector from 4 to 6
bb_0:
  CHECK_TAG R0, ttable, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  CHECK_TAG R1, tnumber, bb_fallback_3
  %8 = LOAD_POINTER R0
  %9 = LOAD_DOUBLE R1
  %10 = TRY_NUM_TO_INDEX %9, bb_fallback_3
  %11 = SUB_INT %10, 1i
  CHECK_ARRAY_SIZE %8, %11, bb_fallback_3
  CHECK_NO_METATABLE %8, bb_fallback_3
  %14 = GET_ARR_ADDR %8, %11
  %15 = LOAD_TVALUE %14
  STORE_TVALUE R2, %15
  JUMP bb_4
bb_4:
  CHECK_TAG R2, ttable, bb_fallback_5
  %23 = LOAD_POINTER R2
  %24 = GET_SLOT_NODE_ADDR %23, 1u, K0 ('normal')
  CHECK_SLOT_MATCH %24, K0 ('normal'), bb_fallback_5
  %26 = LOAD_TVALUE %24, 0i
  STORE_TVALUE R2, %26
  JUMP bb_6
bb_6:
  %31 = LOAD_TVALUE K1 (0.707000017, 0, 0.707000017), 0i, tvector
  STORE_TVALUE R4, %31
  CHECK_TAG R2, tvector, exit(4)
  %37 = LOAD_FLOAT R2, 0i
  %39 = MUL_FLOAT %37, 0.7070000171661377
  %40 = LOAD_FLOAT R2, 4i
  %42 = MUL_FLOAT %40, 0
  %43 = LOAD_FLOAT R2, 8i
  %45 = MUL_FLOAT %43, 0.7070000171661377
  %46 = ADD_FLOAT %39, %42
  %47 = ADD_FLOAT %46, %45
  %48 = FLOAT_TO_NUM %47
  STORE_DOUBLE R2, %48
  STORE_TAG R2, tnumber
  ADJUST_STACK_TO_REG R2, 1i
  INTERRUPT 7u
  RETURN R2, -1i
"#;

    assert_eq!(actual, expected);
}
