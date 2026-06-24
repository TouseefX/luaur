//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:227:ast_query_ast_ancestry_at_eof`
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
//!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
//!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_query_ast_ancestry_at_eof

#[cfg(test)]
#[test]
fn ast_query_ast_ancestry_at_eof() {
    use crate::tests::ast_query_support::*;

    let mut fixture = Fixture::default();
    fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
if true then
    "#,
        ),
        None,
    );

    let source_module = fixture.get_main_source_module();
    let ancestry = unsafe {
        find_ast_ancestry_of_position(&*source_module, Position { line: 2, column: 4 }, false)
    };

    assert!(ancestry.len() >= 2);
    let parent_stat = ancestry[ancestry.len() - 2];
    assert!(unsafe { !ast_node_as::<AstStatIf>(parent_stat).is_null() });
}
