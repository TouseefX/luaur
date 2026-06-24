//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:652:ast_json_encoder_encode_ast_type_optional`
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
//!   - type_ref -> record AstTypeUnion (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeOptional (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_type_optional

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_type_optional() {
    let mut fixture = JsonEncoderFixture::new();
    let root = fixture.expect_parse(
        r#"
            type Foo = string?
        "#,
    );

    assert_eq!(1, unsafe { (*root).body.size });

    assert_eq!(
        json(unsafe { block_statement(root, 0) }),
        r#"{"type":"AstStatTypeAlias","location":"1,12 - 1,30","name":"Foo","generics":[],"genericPacks":[],"value":{"type":"AstTypeUnion","location":"1,23 - 1,30","types":[{"type":"AstTypeReference","location":"1,23 - 1,29","name":"string","nameLocation":"1,23 - 1,29","parameters":[]},{"type":"AstTypeOptional","location":"1,29 - 1,30"}]},"exported":false}"#
    );
}
