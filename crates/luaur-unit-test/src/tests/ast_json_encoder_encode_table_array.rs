//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:147:ast_json_encoder_encode_table_array`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - calls -> method JsonEncoderFixture::expectParse (tests/AstJsonEncoder.test.cpp)
//!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstTypeTable (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_table_array

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_table_array() {
    let mut fixture = JsonEncoderFixture::new();
    let root = fixture.expect_parse(r#"type X = {string}"#);

    assert_eq!(
        json(root),
        desugared_array_type_reference_is_empty(
            r#"{"type":"AstStatBlock","location":"0,0 - 0,17","hasEnd":true,"body":[{"type":"AstStatTypeAlias","location":"0,0 - 0,17","name":"X","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,9 - 0,17","props":[],"indexer":{"location":"0,10 - 0,16","indexType":{"type":"AstTypeReference","location":"0,9 - 0,9","name":"number","nameLocation":"0,9 - 0,9","parameters":[]},"resultType":{"type":"AstTypeReference","location":"0,10 - 0,16","name":"string","nameLocation":"0,10 - 0,16","parameters":[]}}},"exported":false}]}"#,
            r#"{"type":"AstStatBlock","location":"0,0 - 0,17","hasEnd":true,"body":[{"type":"AstStatTypeAlias","location":"0,0 - 0,17","name":"X","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,9 - 0,17","props":[],"indexer":{"location":"0,10 - 0,16","indexType":{"type":"AstTypeReference","location":"0,10 - 0,16","name":"number","nameLocation":"0,10 - 0,16","parameters":[]},"resultType":{"type":"AstTypeReference","location":"0,10 - 0,16","name":"string","nameLocation":"0,10 - 0,16","parameters":[]}}},"exported":false}]}"#,
        )
    );
}
