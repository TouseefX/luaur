//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2395:autocomplete_keyword_members`
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
//!   - translates_to -> rust_item autocomplete_keyword_members

#[cfg(test)]
#[test]
fn autocomplete_keyword_members() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local a = { done = 1, forever = 2 }
local b = a.do@1
local c = a.for@2
local d = a.@3
do
end
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert_eq!(2, ac1.entry_map.len());
    assert!(ac1.entry_map.contains_key("done"));
    assert!(ac1.entry_map.contains_key("forever"));

    let ac2 = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);

    assert_eq!(2, ac2.entry_map.len());
    assert!(ac2.entry_map.contains_key("done"));
    assert!(ac2.entry_map.contains_key("forever"));

    let ac3 = fixture.base.autocomplete_marker(b'3' as core::ffi::c_char);

    assert_eq!(2, ac3.entry_map.len());
    assert!(ac3.entry_map.contains_key("done"));
    assert!(ac3.entry_map.contains_key("forever"));
}
