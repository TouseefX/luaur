//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4477:autocomplete_autocomplete_after_semicolon_should_complete_a_new_statement`
//! Source: `tests/Autocomplete.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Autocomplete.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
//!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Autocomplete.test.cpp
//! - outgoing:
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
//!   - translates_to -> rust_item autocomplete_autocomplete_after_semicolon_should_complete_a_new_statement

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_after_semicolon_should_complete_a_new_statement() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local data = { x = 1 }
local var = data;@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(!ac.entry_map.is_empty());
    assert!(ac.entry_map.contains_key("table"));
    assert!(ac.entry_map.contains_key("math"));
    assert_eq!(ac.context, AutocompleteContext::Statement);
}
