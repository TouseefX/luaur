//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3649:ir_lowering_resolvable_function_returns`
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
//!   - translates_to -> rust_item ir_lowering_resolvable_function_returns

#[cfg(test)]
#[test]
fn ir_lowering_resolvable_function_returns() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
type Vertex = { p: vector, uv: vector, n: vector, t: vector, b: vector, h: number }
local mesh: { vertices: {Vertex}, indices: {number} } = ...

local function temp(b: vector, c: vector) : number
    return 1 / (b.X * c.Y - c.X * b.Y)
end

local function compute()
    for i = 1,#mesh.indices,3 do
        local a = mesh.vertices[mesh.indices[i]]
        local b = mesh.vertices[mesh.indices[i + 1]]
        local c = mesh.vertices[mesh.indices[i + 2]]

        local uvba = b.uv - a.uv
        local uvca = c.uv - a.uv

        local r = temp(uvba, uvca);

        a.t += a.p * r
    end
end
"#,
    )
    .unwrap();

    let actual = format!("\n{}", fixture.get_codegen_header(source.as_ptr()));
    let expected = r#"
; function compute() line 9
; U0: table ['mesh']
; R2: number from 0 to 63 [local 'i']
; R3: table from 7 to 63 [local 'a']
; R4: table from 15 to 63 [local 'b']
; R5: table from 24 to 63 [local 'c']
; R6: vector from 43 to 55 [local 'b']
; R6: vector from 33 to 63 [local 'uvba']
; R7: vector from 37 to 38
; R7: vector from 43 to 55 [local 'c']
; R7: vector from 38 to 63 [local 'uvca']
; R8: vector from 37 to 38
; R8: vector from 42 to 43
; R8: number from 43 to 63 [local 'r']
; R9: vector from 42 to 43
; R9: vector from 60 to 61
; R10: vector from 60 to 61
; R11: vector from 59 to 60
"#;

    assert_eq!(actual, expected);
}
