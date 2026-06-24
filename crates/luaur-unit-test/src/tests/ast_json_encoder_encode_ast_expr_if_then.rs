//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:216:ast_json_encoder_encode_ast_expr_if_then`
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
//!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprIfElse (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_if_then

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_expr_if_then() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("local a = if x then y else z");

    assert_eq!(
        json(statement),
        const2(
            r#"{"type":"AstStatLocal","location":"0,0 - 0,28","vars":[{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,6 - 0,7"}],"values":[{"type":"AstExprIfElse","location":"0,10 - 0,28","condition":{"type":"AstExprGlobal","location":"0,13 - 0,14","global":"x"},"hasThen":true,"trueExpr":{"type":"AstExprGlobal","location":"0,20 - 0,21","global":"y"},"hasElse":true,"falseExpr":{"type":"AstExprGlobal","location":"0,27 - 0,28","global":"z"}}]}"#,
            r#"{"type":"AstStatLocal","location":"0,0 - 0,28","vars":[{"luauType":null,"name":"a","type":"AstLocal","location":"0,6 - 0,7"}],"values":[{"type":"AstExprIfElse","location":"0,10 - 0,28","condition":{"type":"AstExprGlobal","location":"0,13 - 0,14","global":"x"},"hasThen":true,"trueExpr":{"type":"AstExprGlobal","location":"0,20 - 0,21","global":"y"},"hasElse":true,"falseExpr":{"type":"AstExprGlobal","location":"0,27 - 0,28","global":"z"}}]}"#,
        )
    );
}
