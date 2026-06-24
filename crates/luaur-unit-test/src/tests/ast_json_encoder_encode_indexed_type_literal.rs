//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:545:ast_json_encoder_encode_indexed_type_literal`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstTypeTable (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeSingletonBool (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_indexed_type_literal

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_indexed_type_literal() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement(r#"type StringSet = { [string]: true }"#);

    assert_eq!(
        json(statement),
        r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,35","name":"StringSet","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,17 - 0,35","props":[],"indexer":{"location":"0,19 - 0,33","indexType":{"type":"AstTypeReference","location":"0,20 - 0,26","name":"string","nameLocation":"0,20 - 0,26","parameters":[]},"resultType":{"type":"AstTypeSingletonBool","location":"0,29 - 0,33","value":true}}},"exported":false}"#
    );
}
