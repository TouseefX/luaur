//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5002:autocomplete_autocomplete_using_indexer_with_singleton_keys`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item autocomplete_autocomplete_using_indexer_with_singleton_keys

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_using_indexer_with_singleton_keys() {
    use crate::records::ac_fixture::AcFixture;
    use alloc::string::String;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        type List = "Val1" | "Val2" | "Val3"
        local Table: { [List]: boolean }
        local _ = Table.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(ac.entry_map.contains_key("Val1"), true);
    assert_eq!(ac.entry_map.contains_key("Val2"), true);
    assert_eq!(ac.entry_map.contains_key("Val3"), true);
}
