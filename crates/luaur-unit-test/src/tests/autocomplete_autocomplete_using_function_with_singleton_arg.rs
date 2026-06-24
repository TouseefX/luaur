//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5040:autocomplete_autocomplete_using_function_with_singleton_arg`
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
//!   - translates_to -> rust_item autocomplete_autocomplete_using_function_with_singleton_arg

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_using_function_with_singleton_arg() {
    use crate::records::ac_fixture::AcFixture;
    use alloc::string::String;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        local function foo(...: "Val1") end
        foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(ac.entry_map.contains_key("\"Val1\""), true);
}
