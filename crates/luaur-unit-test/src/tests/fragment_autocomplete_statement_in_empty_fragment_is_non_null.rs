//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1190:fragment_autocomplete_statement_in_empty_fragment_is_non_null`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
//!   - calls -> method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item fragment_autocomplete_statement_in_empty_fragment_is_non_null

#[cfg(test)]
#[test]
fn fragment_autocomplete_statement_in_empty_fragment_is_non_null() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::position::Position;
    use luaur_ast::rtti::ast_node_as;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteFixture::default();
    let source = String::from(
        r#"

"#,
    );
    let result = fixture.base.check_with_options(&source);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let fragment = fixture
        .base
        .parse_fragment(&source, &Position { line: 1, column: 0 }, None)
        .expect("expected fragment parse result");

    assert_eq!("", fragment.fragment_to_parse);
    assert_eq!(1, fragment.ancestry.len());
    assert!(!fragment.root.is_null());
    assert_eq!(0, unsafe { (*fragment.root).body.size });
    let stat_body = unsafe { ast_node_as::<AstStatBlock>(fragment.root as *mut AstNode) };
    assert!(!stat_body.is_null());
}
