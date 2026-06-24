//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4895:autocomplete_autocomplete_suggest_hot_comments`
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
//!   - translates_to -> rust_item autocomplete_autocomplete_suggest_hot_comments

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_suggest_hot_comments() {
    use crate::records::ac_fixture::AcFixture;
    use alloc::string::String;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from("--!@1"));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(!ac.entry_map.is_empty());
    assert!(ac.entry_map.contains_key("strict"));
    assert!(ac.entry_map.contains_key("nonstrict"));
    assert!(ac.entry_map.contains_key("nocheck"));
    assert!(ac.entry_map.contains_key("native"));
    assert!(ac.entry_map.contains_key("nolint"));
    assert!(ac.entry_map.contains_key("optimize"));
}
