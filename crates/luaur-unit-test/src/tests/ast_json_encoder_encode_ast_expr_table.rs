//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:305:ast_json_encoder_encode_ast_expr_table`
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
//!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
//!   - calls -> method JsonEncoderFixture::expectParseExpr (tests/AstJsonEncoder.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record AstExprTable (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprConstantString (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_table

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_expr_table() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_expr("{true, key=true, [key2]=true}");

    assert_eq!(
        json(expr),
        r#"{"type":"AstExprTable","location":"0,4 - 0,33","items":[{"type":"AstExprTableItem","kind":"item","value":{"type":"AstExprConstantBool","location":"0,5 - 0,9","value":true}},{"type":"AstExprTableItem","kind":"record","key":{"type":"AstExprConstantString","location":"0,11 - 0,14","value":"key"},"value":{"type":"AstExprConstantBool","location":"0,15 - 0,19","value":true}},{"type":"AstExprTableItem","kind":"general","key":{"type":"AstExprGlobal","location":"0,22 - 0,26","global":"key2"},"value":{"type":"AstExprConstantBool","location":"0,28 - 0,32","value":true}}]}"#
    );
}
