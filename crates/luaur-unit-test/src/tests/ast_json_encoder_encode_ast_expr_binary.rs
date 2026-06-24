//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:325:ast_json_encoder_encode_ast_expr_binary`
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
//!   - type_ref -> record AstExprBinary (Ast/include/Luau/Ast.h)
//!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_binary

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_expr_binary() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_expr("b + c");

    assert_eq!(
        json(expr),
        r#"{"type":"AstExprBinary","location":"0,4 - 0,9","op":"Add","left":{"type":"AstExprGlobal","location":"0,4 - 0,5","global":"b"},"right":{"type":"AstExprGlobal","location":"0,8 - 0,9","global":"c"}}"#
    );
}
