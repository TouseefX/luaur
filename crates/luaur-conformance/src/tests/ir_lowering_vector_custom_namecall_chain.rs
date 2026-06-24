//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1423:ir_lowering_vector_custom_namecall_chain`
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
//!   - translates_to -> rust_item ir_lowering_vector_custom_namecall_chain

#[cfg(test)]
#[test]
fn ir_lowering_vector_custom_namecall_chain() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
    let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(n: vector, b: vector, t: vector)
    return n:Cross(t):Dot(b) + 1
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
  CHECK_TAG R0, tvector, exit(entry)
  CHECK_TAG R1, tvector, exit(entry)
  CHECK_TAG R2, tvector, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %8 = LOAD_TVALUE R2, 0i, tvector
  %14 = LOAD_FLOAT R0, 0i
  %15 = EXTRACT_VEC %8, 0i
  %16 = LOAD_FLOAT R0, 4i
  %17 = EXTRACT_VEC %8, 1i
  %18 = LOAD_FLOAT R0, 8i
  %19 = EXTRACT_VEC %8, 2i
  %20 = MUL_FLOAT %16, %19
  %21 = MUL_FLOAT %18, %17
  %22 = SUB_FLOAT %20, %21
  %23 = MUL_FLOAT %18, %15
  %24 = MUL_FLOAT %14, %19
  %25 = SUB_FLOAT %23, %24
  %26 = MUL_FLOAT %14, %17
  %27 = MUL_FLOAT %16, %15
  %28 = SUB_FLOAT %26, %27
  STORE_VECTOR R4, %22, %25, %28
  STORE_TAG R4, tvector
  %31 = LOAD_TVALUE R1, 0i, tvector
  %37 = LOAD_FLOAT R4, 0i
  %38 = EXTRACT_VEC %31, 0i
  %39 = MUL_FLOAT %37, %38
  %40 = LOAD_FLOAT R4, 4i
  %41 = EXTRACT_VEC %31, 1i
  %42 = MUL_FLOAT %40, %41
  %43 = LOAD_FLOAT R4, 8i
  %44 = EXTRACT_VEC %31, 2i
  %45 = MUL_FLOAT %43, %44
  %46 = ADD_FLOAT %39, %42
  %47 = ADD_FLOAT %46, %45
  %48 = FLOAT_TO_NUM %47
  %54 = ADD_NUM %48, 1
  STORE_DOUBLE R3, %54
  STORE_TAG R3, tnumber
  INTERRUPT 11u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
