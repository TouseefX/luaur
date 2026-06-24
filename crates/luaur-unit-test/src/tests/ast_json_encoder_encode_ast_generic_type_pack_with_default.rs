//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:638:ast_json_encoder_encode_ast_generic_type_pack_with_default`
//! Source: `tests/AstJsonEncoder.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/AstJsonEncoder.test.cpp
//! - source_includes:
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/AstJsonEncoder.test.cpp
//! - outgoing:
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - calls -> method JsonEncoderFixture::expectParse (tests/AstJsonEncoder.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstGenericTypePack (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypePackVariadic (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_generic_type_pack_with_default

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_generic_type_pack_with_default() {
    let mut fixture = JsonEncoderFixture::new();
    let root = fixture.expect_parse(
        r#"
        type Foo<X... = ...string> = any
    "#,
    );

    assert_eq!(1, unsafe { (*root).body.size });

    assert_eq!(
        json(unsafe { block_statement(root, 0) }),
        r#"{"type":"AstStatTypeAlias","location":"1,8 - 1,40","name":"Foo","generics":[],"genericPacks":[{"type":"AstGenericTypePack","name":"X","luauType":{"type":"AstTypePackVariadic","location":"1,24 - 1,33","variadicType":{"type":"AstTypeReference","location":"1,27 - 1,33","name":"string","nameLocation":"1,27 - 1,33","parameters":[]}}}],"value":{"type":"AstTypeReference","location":"1,37 - 1,40","name":"any","nameLocation":"1,37 - 1,40","parameters":[]},"exported":false}"#
    );
}
