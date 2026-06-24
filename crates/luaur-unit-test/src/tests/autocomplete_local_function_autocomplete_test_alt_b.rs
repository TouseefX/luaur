//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1107:autocomplete_local_function`
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
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item autocomplete_local_function

#[cfg(test)]
#[test]
fn autocomplete_local_function() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        local function @1
    "#,
    ));

    let mut ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac.entry_map.is_empty());

    fixture.base.check(&String::from(
        r#"
        local function @1s@2
    "#,
    ));

    ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac.entry_map.is_empty());

    ac = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);
    assert!(ac.entry_map.is_empty());

    fixture.base.check(&String::from(
        r#"
        local function @1()@2
    "#,
    ));

    ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac.entry_map.is_empty());

    ac = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);
    assert!(ac.entry_map.contains_key("end"));

    fixture.base.check(&String::from(
        r#"
        local function something@1
    "#,
    ));

    ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac.entry_map.is_empty());

    fixture.base.check(&String::from(
        r#"
        local tbl = {}
        function tbl.something@1() end
    "#,
    ));

    ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac.entry_map.is_empty());
}
