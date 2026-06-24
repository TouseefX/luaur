//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3803:ir_lowering_unary_type_resolve`
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
//!   - calls -> method LoweringFixture::getCodegenHeader (tests/IrLowering.test.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item ir_lowering_unary_type_resolve

#[cfg(test)]
#[test]
fn ir_lowering_unary_type_resolve() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a, b: vector, c)
    local d = not a
    local e = -b
    local f = #c
    return (if d then e else vector(f, 2, 3)).X
end
"#,
    )
    .unwrap();

    let actual = format!("\n{}", fixture.get_codegen_header(source.as_ptr()));
    let expected = r#"
; function foo(a, b, c) line 2
; R1: vector [argument 'b']
; R3: boolean from 0 to 17 [local 'd']
; R4: vector from 1 to 17 [local 'e']
; R5: number from 2 to 17 [local 'f']
; R6: vector from 14 to 16
"#;

    assert_eq!(actual, expected);
}
