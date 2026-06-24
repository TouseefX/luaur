//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:264:ast_json_encoder_encode_ast_expr_call`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record AstExprCall (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - type_ref -> record AstExprConstantNumber (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_call

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_expr_call() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_expr("foo(1, 2, 3)");

    assert_eq!(
        json(expr),
        r#"{"type":"AstExprCall","location":"0,4 - 0,16","func":{"type":"AstExprGlobal","location":"0,4 - 0,7","global":"foo"},"args":[{"type":"AstExprConstantNumber","location":"0,8 - 0,9","value":1},{"type":"AstExprConstantNumber","location":"0,11 - 0,12","value":2},{"type":"AstExprConstantNumber","location":"0,14 - 0,15","value":3}],"self":false,"argLocation":"0,8 - 0,16"}"#
    );
}
