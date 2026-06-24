//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:390:ast_query_find_binding_at_position_global_start_of_file`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - calls -> function findBindingAtPosition (Analysis/src/AstQuery.cpp)
//!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
//!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item ast_query_find_binding_at_position_global_start_of_file

#[cfg(test)]
#[test]
fn ast_query_find_binding_at_position_global_start_of_file() {
    use crate::tests::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    fixture.base.get_frontend();
    fixture
        .base
        .base
        .check_string_optional_frontend_options(&String::from("local x = string.char(1)"), None);
    let pos = Position {
        line: 0,
        column: 12,
    };

    let module = fixture.base.base.get_main_module(false);
    let source_module = fixture.base.base.get_main_source_module();
    let binding = unsafe { find_binding_at_position(&*module, &*source_module, pos) };

    assert!(binding.is_some());
    assert_eq!(
        binding.unwrap().location,
        Location {
            begin: Position { line: 0, column: 0 },
            end: Position { line: 0, column: 0 },
        }
    );
}
