//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1484:ir_lowering_vector_custom_namecall_chain_2`
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
//!   - translates_to -> rust_item ir_lowering_vector_custom_namecall_chain_2

#[cfg(test)]
#[test]
fn ir_lowering_vector_custom_namecall_chain_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);
    let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
    let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
type Vertex = {n: vector, b: vector}

local function foo(v: Vertex, t: vector)
    return v.n:Cross(t):Dot(v.b) + 1
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1) line 4
bb_0:
  CHECK_TAG R0, ttable, exit(entry)
  CHECK_TAG R1, tvector, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %8 = LOAD_POINTER R0
  %9 = GET_SLOT_NODE_ADDR %8, 0u, K1 ('n')
  CHECK_SLOT_MATCH %9, K1 ('n'), bb_fallback_3
  %11 = LOAD_TVALUE %9, 0i
  STORE_TVALUE R3, %11
  JUMP bb_4
bb_4:
  %16 = LOAD_TVALUE R1, 0i, tvector
  CHECK_TAG R3, tvector, bb_exit_7
   ; exit sync: R5, {%16}
  %22 = LOAD_FLOAT R3, 0i
  %23 = EXTRACT_VEC %16, 0i
  %24 = LOAD_FLOAT R3, 4i
  %25 = EXTRACT_VEC %16, 1i
  %26 = LOAD_FLOAT R3, 8i
  %27 = EXTRACT_VEC %16, 2i
  %28 = MUL_FLOAT %24, %27
  %29 = MUL_FLOAT %26, %25
  %30 = SUB_FLOAT %28, %29
  %31 = MUL_FLOAT %26, %23
  %32 = MUL_FLOAT %22, %27
  %33 = SUB_FLOAT %31, %32
  %34 = MUL_FLOAT %22, %25
  %35 = MUL_FLOAT %24, %23
  %36 = SUB_FLOAT %34, %35
  STORE_VECTOR R3, %30, %33, %36
  %41 = LOAD_POINTER R0
  %42 = GET_SLOT_NODE_ADDR %41, 7u, K3 ('b')
  CHECK_SLOT_MATCH %42, K3 ('b'), bb_fallback_5
  %44 = LOAD_TVALUE %42, 0i
  STORE_TVALUE R5, %44
  JUMP bb_6
bb_6:
  CHECK_TAG R3, tvector, exit(9)
  CHECK_TAG R5, tvector, exit(9)
  %53 = LOAD_FLOAT R3, 0i
  %54 = LOAD_FLOAT R5, 0i
  %55 = MUL_FLOAT %53, %54
  %56 = LOAD_FLOAT R3, 4i
  %57 = LOAD_FLOAT R5, 4i
  %58 = MUL_FLOAT %56, %57
  %59 = LOAD_FLOAT R3, 8i
  %60 = LOAD_FLOAT R5, 8i
  %61 = MUL_FLOAT %59, %60
  %62 = ADD_FLOAT %55, %58
  %63 = ADD_FLOAT %62, %61
  %64 = FLOAT_TO_NUM %63
  %70 = ADD_NUM %64, 1
  STORE_DOUBLE R2, %70
  STORE_TAG R2, tnumber
  INTERRUPT 14u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
