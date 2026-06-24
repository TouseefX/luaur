//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:295:ast_query_luau_nested_query`
//! Source: `tests/AstQuery.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/AstQuery.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file tests/AstQueryDsl.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/AstQuery.test.cpp
//! - outgoing:
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
//!   - calls -> function query (tests/AstQueryDsl.h)
//!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_query_luau_nested_query

#[cfg(test)]
#[test]
fn ast_query_luau_nested_query() {
    use crate::tests::ast_query_support::*;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        r#"
        if true then
        end
    "#,
        &ParseOptions::default(),
    );

    let if_ = query::<AstStatIf>(block as *mut AstNode, vec![nth_T::<AstStatIf>(1)]);
    assert!(!if_.is_null());

    let bool_ =
        query::<AstExprConstantBool>(if_ as *mut AstNode, vec![nth_T::<AstExprConstantBool>(1)]);
    assert!(!bool_.is_null());
}
