//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:362:ast_query_find_name_ancestry`
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
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
//!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
//!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_query_find_name_ancestry

#[cfg(test)]
#[test]
fn ast_query_find_name_ancestry() {
    use crate::tests::ast_query_support::*;

    let mut fixture = Fixture::default();
    fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local tbl = {}
        function tbl:abc() end
    "#,
        ),
        None,
    );
    let pos = Position {
        line: 2,
        column: 18,
    };

    let source_module = fixture.get_main_source_module();
    let ancestry = unsafe { find_ast_ancestry_of_position(&*source_module, pos, false) };

    assert!(!ancestry.is_empty());
    assert!(unsafe { !ast_node_as::<AstExprLocal>(*ancestry.last().unwrap()).is_null() });
}
