//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:349:ast_query_include_types_ancestry`
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
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
//!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
//!   - translates_to -> rust_item ast_query_include_types_ancestry

#[cfg(test)]
#[test]
fn ast_query_include_types_ancestry() {
    use crate::tests::ast_query_support::*;

    let mut fixture = Fixture::default();
    fixture.check_string_optional_frontend_options(&String::from("local x: number = 4;"), None);
    let pos = Position {
        line: 0,
        column: 10,
    };

    let source_module = fixture.get_main_source_module();
    let ancestry_no_types = unsafe { find_ast_ancestry_of_position(&*source_module, pos, false) };
    let ancestry_types = unsafe { find_ast_ancestry_of_position(&*source_module, pos, true) };

    assert!(ancestry_types.len() > ancestry_no_types.len());
    assert!(unsafe {
        (*ancestry_no_types.last().copied().unwrap())
            .as_type()
            .is_null()
    });
    assert!(unsafe {
        !(*ancestry_types.last().copied().unwrap())
            .as_type()
            .is_null()
    });
}
