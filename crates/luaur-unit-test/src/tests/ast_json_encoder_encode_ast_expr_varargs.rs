//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:257:ast_json_encoder_encode_ast_expr_varargs`
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
//!   - type_ref -> record AstExprVarargs (Ast/include/Luau/Ast.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_varargs

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_expr_varargs() {
    let mut varargs = AstExprVarargs::new(Location::default());

    assert_eq!(
        json_ref(&mut varargs),
        r#"{"type":"AstExprVarargs","location":"0,0 - 0,0"}"#
    );
}
