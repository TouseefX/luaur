//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4776:autocomplete_autocomplete_exclude_break_continue_outside_loop`
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
//!   - translates_to -> rust_item autocomplete_autocomplete_exclude_break_continue_outside_loop

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_exclude_break_continue_outside_loop() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"@1if true then
        @2
    end"#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(ac1.entry_map.get("break").is_some(), false);
    assert_eq!(ac1.entry_map.get("continue").is_some(), false);

    let ac2 = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);
    assert_eq!(ac2.entry_map.get("break").is_some(), false);
    assert_eq!(ac2.entry_map.get("continue").is_some(), false);
}
