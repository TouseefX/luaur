//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4923:autocomplete_autocomplete_method_in_unfinished_repeat_body_not_eof`
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
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - translates_to -> rust_item autocomplete_autocomplete_method_in_unfinished_repeat_body_not_eof

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_method_in_unfinished_repeat_body_not_eof() {
    use crate::records::ac_fixture::AcFixture;
    use alloc::string::String;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"local t = {}
        function t:Foo() end
        repeat
        t:@1
        "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(!ac.entry_map.is_empty());
    assert!(ac.entry_map.contains_key("Foo"));
}
