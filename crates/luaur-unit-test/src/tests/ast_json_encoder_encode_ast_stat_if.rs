//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:363:ast_json_encoder_encode_ast_stat_if`
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
//!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_if

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_stat_if() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("if true then else end");

    assert_eq!(
        json(statement),
        r#"{"type":"AstStatIf","location":"0,0 - 0,21","condition":{"type":"AstExprConstantBool","location":"0,3 - 0,7","value":true},"thenbody":{"type":"AstStatBlock","location":"0,12 - 0,13","hasEnd":true,"body":[]},"elsebody":{"type":"AstStatBlock","location":"0,17 - 0,18","hasEnd":true,"body":[]},"hasThen":true}"#
    );
}
