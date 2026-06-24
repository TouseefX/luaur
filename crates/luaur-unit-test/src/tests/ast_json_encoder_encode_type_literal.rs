//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:533:ast_json_encoder_encode_type_literal`
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
//!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
//!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstTypeTable (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTableProp (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeUnion (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeSingletonString (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeSingletonBool (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_type_literal

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_type_literal() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement(
        r#"type Action = { strings: "A" | "B" | "C", mixed: "This" | "That" | true }"#,
    );

    assert_eq!(
        json(statement),
        r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,73","name":"Action","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,14 - 0,73","props":[{"name":"strings","type":"AstTableProp","location":"0,16 - 0,23","propType":{"type":"AstTypeUnion","location":"0,25 - 0,40","types":[{"type":"AstTypeSingletonString","location":"0,25 - 0,28","value":"A"},{"type":"AstTypeSingletonString","location":"0,31 - 0,34","value":"B"},{"type":"AstTypeSingletonString","location":"0,37 - 0,40","value":"C"}]}},{"name":"mixed","type":"AstTableProp","location":"0,42 - 0,47","propType":{"type":"AstTypeUnion","location":"0,49 - 0,71","types":[{"type":"AstTypeSingletonString","location":"0,49 - 0,55","value":"This"},{"type":"AstTypeSingletonString","location":"0,58 - 0,64","value":"That"},{"type":"AstTypeSingletonBool","location":"0,67 - 0,71","value":true}]}}],"indexer":null},"exported":false}"#
    );
}
