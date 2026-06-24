//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:1295:ir_lowering_vector_custom_namecall`
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
//!   - translates_to -> rust_item ir_lowering_vector_custom_namecall

#[cfg(test)]
#[test]
fn ir_lowering_vector_custom_namecall() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
    let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function vec3dot(a: vector, b: vector)
    return (a:Dot(b))
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function vec3dot($arg0, $arg1) line 2
bb_0:
  CHECK_TAG R0, tvector, exit(entry)
  CHECK_TAG R1, tvector, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %6 = LOAD_TVALUE R1, 0i, tvector
  %12 = LOAD_FLOAT R0, 0i
  %13 = EXTRACT_VEC %6, 0i
  %14 = MUL_FLOAT %12, %13
  %15 = LOAD_FLOAT R0, 4i
  %16 = EXTRACT_VEC %6, 1i
  %17 = MUL_FLOAT %15, %16
  %18 = LOAD_FLOAT R0, 8i
  %19 = EXTRACT_VEC %6, 2i
  %20 = MUL_FLOAT %18, %19
  %21 = ADD_FLOAT %14, %17
  %22 = ADD_FLOAT %21, %20
  %23 = FLOAT_TO_NUM %22
  STORE_DOUBLE R2, %23
  STORE_TAG R2, tnumber
  INTERRUPT 5u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
