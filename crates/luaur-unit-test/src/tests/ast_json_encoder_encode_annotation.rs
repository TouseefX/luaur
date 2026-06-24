//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:524:ast_json_encoder_encode_annotation`
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
//!   - type_ref -> record AstTypeIntersection (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeGroup (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeFunction (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeList (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypePackExplicit (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeUnion (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_annotation

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_annotation() {
    let mut fixture = JsonEncoderFixture::new();
    let statement =
        fixture.expect_parse_statement("type T = ((number) -> (string | nil)) & ((string) -> ())");

    assert_eq!(
        json(statement),
        r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,56","name":"T","generics":[],"genericPacks":[],"value":{"type":"AstTypeIntersection","location":"0,9 - 0,56","types":[{"type":"AstTypeGroup","location":"0,9 - 0,37","inner":{"type":"AstTypeFunction","location":"0,10 - 0,36","attributes":[],"generics":[],"genericPacks":[],"argTypes":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,11 - 0,17","name":"number","nameLocation":"0,11 - 0,17","parameters":[]}]},"argNames":[],"returnTypes":{"type":"AstTypePackExplicit","location":"0,22 - 0,36","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeGroup","location":"0,22 - 0,36","inner":{"type":"AstTypeUnion","location":"0,23 - 0,35","types":[{"type":"AstTypeReference","location":"0,23 - 0,29","name":"string","nameLocation":"0,23 - 0,29","parameters":[]},{"type":"AstTypeReference","location":"0,32 - 0,35","name":"nil","nameLocation":"0,32 - 0,35","parameters":[]}]}}]}}}},{"type":"AstTypeGroup","location":"0,40 - 0,56","inner":{"type":"AstTypeFunction","location":"0,41 - 0,55","attributes":[],"generics":[],"genericPacks":[],"argTypes":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,42 - 0,48","name":"string","nameLocation":"0,42 - 0,48","parameters":[]}]},"argNames":[],"returnTypes":{"type":"AstTypePackExplicit","location":"0,53 - 0,55","typeList":{"type":"AstTypeList","types":[]}}}}]},"exported":false}"#
    );
}
