//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1171:fragment_autocomplete_thrown_parse_error_leads_to_null_root`
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
//!   - calls -> method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - calls -> method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item fragment_autocomplete_thrown_parse_error_leads_to_null_root

#[cfg(test)]
#[test]
fn fragment_autocomplete_thrown_parse_error_leads_to_null_root() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_ast::records::position::Position;
    use luaur_common::FInt;

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.check_with_options(&String::from("type A =  "));
    let _sfi = ScopedFastInt::new(&FInt::LuauParseErrorLimit, 1);
    let fragment = fixture.base.parse_fragment(
        &String::from("type A = <>function<> more garbage here"),
        &Position {
            line: 0,
            column: 39,
        },
        None,
    );

    assert!(fragment.is_none());
}
