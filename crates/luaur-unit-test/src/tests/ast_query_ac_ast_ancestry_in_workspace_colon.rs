//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:262:ast_query_ac_ast_ancestry_in_workspace_colon`
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
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
//!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_query_ac_ast_ancestry_in_workspace_colon

#[cfg(test)]
#[test]
fn ast_query_ac_ast_ancestry_in_workspace_colon() {
    use crate::tests::ast_query_support::*;

    let mut fixture = Fixture::default();
    fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
print(workspace:)
    "#,
        ),
        None,
    );

    let source_module = fixture.get_main_source_module();
    let ancestry = unsafe {
        find_ancestry_at_position_for_autocomplete(
            &*source_module,
            Position {
                line: 1,
                column: 16,
            },
        )
    };

    assert!(ancestry.len() >= 2);
    assert!(unsafe { !ast_node_as::<AstExprIndexName>(*ancestry.last().unwrap()).is_null() });
}
