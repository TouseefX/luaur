//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3636:autocomplete_string_prim_non_self_calls_are_avoided`
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
//!   - translates_to -> rust_item autocomplete_string_prim_non_self_calls_are_avoided

#[cfg(test)]
#[test]
fn autocomplete_string_prim_non_self_calls_are_avoided() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local s = "hello"
s.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("char"));
    assert!(!ac.entry_map["char"].wrong_index_type);
    assert!(!ac.entry_map["char"].indexed_with_self);
    assert!(ac.entry_map.contains_key("sub"));
    assert!(ac.entry_map["sub"].wrong_index_type);
    assert!(!ac.entry_map["sub"].indexed_with_self);
}
