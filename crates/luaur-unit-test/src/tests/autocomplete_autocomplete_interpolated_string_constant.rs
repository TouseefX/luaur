//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2904:autocomplete_autocomplete_interpolated_string_constant`
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
//!   - translates_to -> rust_item autocomplete_autocomplete_interpolated_string_constant

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_interpolated_string_constant() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;

    let mut fixture = AcFixture::default();

    fixture.base.check(&String::from(r#"f(`@1`)"#));
    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac1.entry_map.is_empty());
    assert_eq!(ac1.context, AutocompleteContext::String);

    fixture.base.check(&String::from(r#"f(`@1 {"a"}`)"#));
    let ac2 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac2.entry_map.is_empty());
    assert_eq!(ac2.context, AutocompleteContext::String);

    fixture.base.check(&String::from(r#"f(`{"a"} @1`)"#));
    let ac3 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac3.entry_map.is_empty());
    assert_eq!(ac3.context, AutocompleteContext::String);

    fixture.base.check(&String::from(r#"f(`{"a"} @1 {"b"}`)"#));
    let ac4 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac4.entry_map.is_empty());
    assert_eq!(ac4.context, AutocompleteContext::String);
}
