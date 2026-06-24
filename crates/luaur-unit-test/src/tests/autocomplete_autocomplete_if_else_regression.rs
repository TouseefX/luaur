//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2884:autocomplete_autocomplete_if_else_regression`
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
//!   - translates_to -> rust_item autocomplete_autocomplete_if_else_regression

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_if_else_regression() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local abcdef = 0;
local temp = false
local even = true;
local a
a = if temp then even else@1
a = if temp then even else @2
a = if temp then even else abc@3
        "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(!ac1.entry_map.contains_key("else"));

    let ac2 = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);
    assert!(!ac2.entry_map.contains_key("else"));

    let ac3 = fixture.base.autocomplete_marker(b'3' as core::ffi::c_char);
    assert!(ac3.entry_map.contains_key("abcdef"));
}
