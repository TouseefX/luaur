//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3748:autocomplete_globals_are_order_independent`
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
//!   - translates_to -> rust_item autocomplete_globals_are_order_independent

#[cfg(test)]
#[test]
fn autocomplete_globals_are_order_independent() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        local myLocal = 4
        function abc0()
            local myInnerLocal = 1
@1
        end

        function abc1()
            local myInnerLocal = 1
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("myLocal"));
    assert!(ac.entry_map.contains_key("myInnerLocal"));
    assert!(ac.entry_map.contains_key("abc0"));
    assert!(ac.entry_map.contains_key("abc1"));
}
