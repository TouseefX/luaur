//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:321:ast_query_luau_selectively_query_for_a_different_boolean`
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
//!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
//!   - calls -> function query (tests/AstQueryDsl.h)
//!   - calls -> function nth (tests/AstQueryDsl.h)
//!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_query_luau_selectively_query_for_a_different_boolean

#[cfg(test)]
#[test]
fn ast_query_luau_selectively_query_for_a_different_boolean() {
    use crate::tests::ast_query_support::*;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        r#"
        local x = false and true
        local y = true and false
    "#,
        &ParseOptions::default(),
    );

    let fst = query::<AstExprConstantBool>(
        block as *mut AstNode,
        vec![nth_T::<AstStatLocal>(1), nth_T::<AstExprConstantBool>(2)],
    );
    assert!(!fst.is_null());
    assert_eq!(true, unsafe { (*fst).value });

    let snd = query::<AstExprConstantBool>(
        block as *mut AstNode,
        vec![nth_T::<AstStatLocal>(2), nth_T::<AstExprConstantBool>(2)],
    );
    assert!(!snd.is_null());
    assert_eq!(false, unsafe { (*snd).value });
}
