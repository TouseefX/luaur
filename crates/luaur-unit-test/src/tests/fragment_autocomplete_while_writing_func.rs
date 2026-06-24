//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:564:fragment_autocomplete_while_writing_func`
//! Source: `tests/FragmentAutocomplete.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
//! - outgoing:
//!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record AstStatFunction (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item fragment_autocomplete_while_writing_func

#[cfg(test)]
#[test]
fn fragment_autocomplete_while_writing_func() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::string::String;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_function::AstStatFunction;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_ast::rtti::ast_node_as;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
        String::from(
            r#"
function f(arg1,
"#,
        ),
        &Position {
            line: 1,
            column: 17,
        },
    );

    assert_eq!(
        Location {
            begin: Position { line: 1, column: 0 },
            end: Position {
                line: 1,
                column: 17
            },
        },
        region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
        !unsafe { ast_node_as::<AstStatFunction>(region.nearest_statement as *mut AstNode) }
            .is_null()
    );
}
