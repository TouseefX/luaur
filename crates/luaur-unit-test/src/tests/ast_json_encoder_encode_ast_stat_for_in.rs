//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:425:ast_json_encoder_encode_ast_stat_for_in`
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
//!   - type_ref -> record AstStatForIn (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_for_in

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_stat_for_in() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("for a in b do end");

    assert_eq!(
        json(statement),
        const2(
            r#"{"type":"AstStatForIn","location":"0,0 - 0,17","vars":[{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,4 - 0,5"}],"values":[{"type":"AstExprGlobal","location":"0,9 - 0,10","global":"b"}],"body":{"type":"AstStatBlock","location":"0,13 - 0,14","hasEnd":true,"body":[]},"hasIn":true,"hasDo":true}"#,
            r#"{"type":"AstStatForIn","location":"0,0 - 0,17","vars":[{"luauType":null,"name":"a","type":"AstLocal","location":"0,4 - 0,5"}],"values":[{"type":"AstExprGlobal","location":"0,9 - 0,10","global":"b"}],"body":{"type":"AstStatBlock","location":"0,13 - 0,14","hasEnd":true,"body":[]},"hasIn":true,"hasDo":true}"#,
        )
    );
}
