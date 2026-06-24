//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3145:autocomplete_string_singleton_as_table_key_iso`
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
//!   - translates_to -> rust_item autocomplete_string_singleton_as_table_key_iso

#[cfg(test)]
#[test]
fn autocomplete_string_singleton_as_table_key_iso() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        type Direction = "up" | "down"
        local b: {[Direction]: boolean} = {["@2"] = true}
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("up"));
    assert!(ac.entry_map.contains_key("down"));
}
