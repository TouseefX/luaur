//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4950:autocomplete_autocomplete_empty_attribute`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item autocomplete_autocomplete_empty_attribute

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_empty_attribute() {
    use crate::records::ac_builtins_fixture::ACBuiltinsFixture;
    use alloc::string::String;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(&String::from(
        r#"
        \@@1
        function foo() return 42 end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(ac.entry_map.contains_key("deprecated"), true);
    assert_eq!(ac.entry_map.contains_key("checked"), true);
    assert_eq!(ac.entry_map.contains_key("native"), true);
}
