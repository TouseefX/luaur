//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:345:ast_json_encoder_encode_ast_expr_error`
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
//!   - type_ref -> record ParseResult (Ast/include/Luau/ParseResult.h)
//!   - type_ref -> record Parser (Ast/include/Luau/Parser.h)
//!   - calls -> method JsonEncoderFixture::parse (tests/AstJsonEncoder.test.cpp)
//!   - type_ref -> record AstStatAssign (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record AstExprError (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_error

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_expr_error() {
    let mut fixture = JsonEncoderFixture::new();
    fixture
        .names
        .rebind_allocator(&mut fixture.allocator as *mut _);
    let src = "a = ";
    let parse_result = Parser::parse(
        src,
        src.len(),
        &mut fixture.names,
        &mut fixture.allocator,
        ParseOptions::default(),
    );

    assert_eq!(1, unsafe { (*parse_result.root).body.size });
    let stat = unsafe { *(*parse_result.root).body.data.add(0) };
    let stat_assign = unsafe { luaur_ast::rtti::ast_node_as::<AstStatAssign>(stat as *mut AstNode) };
    assert!(!stat_assign.is_null());
    assert_eq!(1, unsafe { (*stat_assign).values.size });
    let expr = unsafe { *(*stat_assign).values.data.add(0) };

    assert_eq!(
        json(expr),
        r#"{"type":"AstExprError","location":"0,4 - 0,4","expressions":[],"messageIndex":0}"#
    );
}
