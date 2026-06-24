//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:4010:ir_lowering_custom_userdata_types`
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
//!   - calls -> method LoweringFixture::getCodegenHeader (tests/IrLowering.test.cpp)
//!   - translates_to -> rust_item ir_lowering_custom_userdata_types

#[cfg(test)]
#[test]
fn ir_lowering_custom_userdata_types() {
    use crate::records::lowering_fixture::LoweringFixture;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use std::ffi::CString;

    if luau_codegen_supported() == 0 {
        return;
    }

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(v: vec2, x: mat3)
    return v.X * x
end
"#,
    )
    .unwrap();

    let actual = format!("\n{}", fixture.get_codegen_header(source.as_ptr()));
    let expected = r#"
; function foo(v, x) line 2
; R0: vec2 [argument 'v']
; R1: mat3 [argument 'x']
"#;

    assert_eq!(actual, expected);
}
