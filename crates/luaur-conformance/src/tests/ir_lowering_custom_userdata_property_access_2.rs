//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:4069:ir_lowering_custom_userdata_property_access_2`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function isSupported (CodeGen/src/CodeGen.cpp)
//!   - calls -> method LoweringFixture::getCodegenAssembly (tests/IrLowering.test.cpp)
//!   - translates_to -> rust_item ir_lowering_custom_userdata_property_access_2

#[cfg(test)]
#[test]
fn ir_lowering_custom_userdata_property_access_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use std::ffi::CString;

    if luau_codegen_supported() == 0 {
        return;
    }

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: mat3)
    return a.Row1 * a.Row2
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0) line 2
; R0: mat3 [argument]
bb_0:
  CHECK_TAG R0, tuserdata, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  FALLBACK_GETTABLEKS 0u, R2, R0, K0 ('Row1')
  FALLBACK_GETTABLEKS 2u, R3, R0, K1 ('Row2')
  CHECK_TAG R2, tvector, exit(4)
  CHECK_TAG R3, tvector, exit(4)
  %14 = LOAD_TVALUE R2, 0i, tvector
  %15 = LOAD_TVALUE R3, 0i, tvector
  %16 = MUL_VEC %14, %15
  %17 = TAG_VECTOR %16
  STORE_TVALUE R1, %17
  INTERRUPT 5u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
