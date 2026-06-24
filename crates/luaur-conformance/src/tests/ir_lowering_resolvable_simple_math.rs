//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3593:ir_lowering_resolvable_simple_math`
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
//!   - translates_to -> rust_item ir_lowering_resolvable_simple_math

#[cfg(test)]
#[test]
fn ir_lowering_resolvable_simple_math() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
type Vertex = { p: vector, uv: vector, n: vector, t: vector, b: vector, h: number }
local mesh: { vertices: {Vertex}, indices: {number} } = ...

local function compute()
    for i = 1,#mesh.indices,3 do
        local a = mesh.vertices[mesh.indices[i]]
        local b = mesh.vertices[mesh.indices[i + 1]]
        local c = mesh.vertices[mesh.indices[i + 2]]

        local vba = b.p - a.p
        local vca = c.p - a.p

        local uvba = b.uv - a.uv
        local uvca = c.uv - a.uv

        local r = 1.0 / (uvba.X * uvca.Y - uvca.X * uvba.Y);

        local sdir = (uvca.Y * vba - uvba.Y * vca) * r

        a.t += sdir
    end
end
"#,
    )
    .unwrap();

    let actual = format!("\n{}", fixture.get_codegen_header(source.as_ptr()));
    let expected = r#"
; function compute() line 5
; U0: table ['mesh']
; R2: number from 0 to 78 [local 'i']
; R3: table from 7 to 78 [local 'a']
; R4: table from 15 to 78 [local 'b']
; R5: table from 24 to 78 [local 'c']
; R6: vector from 33 to 78 [local 'vba']
; R7: vector from 37 to 38
; R7: vector from 38 to 78 [local 'vca']
; R8: vector from 37 to 38
; R8: vector from 42 to 43
; R8: vector from 43 to 78 [local 'uvba']
; R9: vector from 42 to 43
; R9: vector from 47 to 48
; R9: vector from 48 to 78 [local 'uvca']
; R10: vector from 47 to 48
; R10: vector from 52 to 53
; R10: number from 53 to 78 [local 'r']
; R11: vector from 52 to 53
; R11: vector from 65 to 78 [local 'sdir']
; R12: vector from 72 to 73
; R12: vector from 75 to 76
; R13: vector from 71 to 72
; R14: vector from 71 to 72
"#;

    assert_eq!(actual, expected);
}
