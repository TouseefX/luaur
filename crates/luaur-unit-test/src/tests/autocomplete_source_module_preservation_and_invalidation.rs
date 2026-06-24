//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3710:autocomplete_source_module_preservation_and_invalidation`
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
//!   - calls -> method ACFixture::getFrontend (tests/Autocomplete.test.cpp)
//!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
//!   - translates_to -> rust_item autocomplete_source_module_preservation_and_invalidation

#[cfg(test)]
#[test]
fn autocomplete_source_module_preservation_and_invalidation() {
    use crate::records::ac_fixture::AcFixture;
    use alloc::string::String;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local a = { x = 2, y = 4 }
a.@1
    "#,
    ));

    fixture.base.get_frontend().clear();

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert_eq!(2, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("x"));
    assert!(ac.entry_map.contains_key("y"));

    fixture
        .base
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("MainModule"), None);

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("x"));
    assert!(ac.entry_map.contains_key("y"));

    fixture
        .base
        .get_frontend()
        .mark_dirty(&String::from("MainModule"), None);

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("x"));
    assert!(ac.entry_map.contains_key("y"));

    fixture
        .base
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("MainModule"), None);

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("x"));
    assert!(ac.entry_map.contains_key("y"));
}
