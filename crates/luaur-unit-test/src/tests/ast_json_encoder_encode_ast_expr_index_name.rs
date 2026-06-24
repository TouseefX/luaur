//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:273:ast_json_encoder_encode_ast_expr_index_name`
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
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
//!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_index_name

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_expr_index_name() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_expr("foo.bar");

    assert_eq!(
        json(expr),
        r#"{"type":"AstExprIndexName","location":"0,4 - 0,11","expr":{"type":"AstExprGlobal","location":"0,4 - 0,7","global":"foo"},"index":"bar","indexLocation":"0,8 - 0,11","op":"."}"#
    );
}
