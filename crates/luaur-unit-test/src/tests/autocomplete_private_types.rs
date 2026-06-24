//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1343:autocomplete_private_types`
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
//!   - translates_to -> rust_item autocomplete_private_types

#[cfg(test)]
#[test]
fn autocomplete_private_types() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
do
    type num = number
    local a: n@1u
    local b: nu@2m
end
local a: nu@3
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac1.entry_map.contains_key("num"));
    assert!(ac1.entry_map.contains_key("number"));

    let ac2 = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);

    assert!(ac2.entry_map.contains_key("num"));
    assert!(ac2.entry_map.contains_key("number"));

    let ac3 = fixture.base.autocomplete_marker(b'3' as core::ffi::c_char);

    assert!(!ac3.entry_map.contains_key("num"));
    assert!(ac3.entry_map.contains_key("number"));
}
