//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1479:autocomplete_function_type_types`
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
//!   - translates_to -> rust_item autocomplete_function_type_types

#[cfg(test)]
#[test]
fn autocomplete_function_type_types() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local a: (n@1
local b: (number, (n@2
local c: (number, (number) -> n@3
local d: (number, (number) -> (number, n@4
local e: (n: n@5
    "#,
    ));

    for marker in b'1'..=b'5' {
        let ac = fixture
            .base
            .autocomplete_marker(marker as core::ffi::c_char);

        assert!(ac.entry_map.contains_key("nil"));
        assert!(ac.entry_map.contains_key("number"));
    }
}
