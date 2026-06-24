//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3983:ir_lowering_for_in_auto_annotation_generic`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Vertex (tests/ConformanceIrHooks.h)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item ir_lowering_for_in_auto_annotation_generic

#[cfg(test)]
#[test]
fn ir_lowering_for_in_auto_annotation_generic() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
type Vertex = {pos: vector, normal: vector}

local function foo(a: {Vertex})
    local sum = 0
    for k, v in a do
        local n = v.pos.X
        sum += n
    end
    return sum
end
"#,
    )
    .unwrap();

    let actual = format!("\n{}", fixture.get_codegen_header(source.as_ptr()));
    let expected = r#"
; function foo(a) line 4
; R0: table [argument 'a']
; R1: number from 0 to 13 [local 'sum']
; R5: number from 4 to 10 [local 'k']
; R6: table from 4 to 10 [local 'v']
; R7: vector from 7 to 9
; R7: number from 5 to 10 [local 'n']
"#;

    assert_eq!(actual, expected);
}
