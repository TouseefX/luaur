//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2492:autocomplete_if_then_else_full_keywords`
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
//!   - translates_to -> rust_item autocomplete_if_then_else_full_keywords

#[cfg(test)]
#[test]
fn autocomplete_if_then_else_full_keywords() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local thenceforth = false
local elsewhere = false
local doover = false
local endurance = true

if 1 then@1
else@2
end

while false do@3
end

repeat@4
until
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(1, ac1.entry_map.len());
    assert!(ac1.entry_map.contains_key("then"));

    let ac2 = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);
    assert!(ac2.entry_map.contains_key("else"));
    assert!(ac2.entry_map.contains_key("elseif"));

    let ac3 = fixture.base.autocomplete_marker(b'3' as core::ffi::c_char);
    assert!(ac3.entry_map.contains_key("do"));

    let ac4 = fixture.base.autocomplete_marker(b'4' as core::ffi::c_char);
    assert!(ac4.entry_map.contains_key("do"));
}
