//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3222:autocomplete_string_singleton_in_if_statement`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record AutocompleteResult (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_string_singleton_in_if_statement

#[cfg(test)]
#[test]
fn autocomplete_string_singleton_in_if_statement() {
    use crate::records::ac_fixture::AcFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        --!strict

        type Direction = "left" | "right"

        local dir: Direction = "left"

        if dir == @1"@2"@3 then end
        local a: {[Direction]: boolean} = {[@4"@5"@6]}

        if dir == @7`@8`@9 then end
        local a: {[Direction]: boolean} = {[@A`@B`@C]}
    "#,
    ));

    let mut check_marker = |marker: u8, has_left: bool, has_right: bool| {
        let ac = fixture
            .base
            .autocomplete_marker(marker as core::ffi::c_char);
        assert_eq!(ac.entry_map.contains_key("left"), has_left);
        assert_eq!(ac.entry_map.contains_key("right"), has_right);
    };

    check_marker(b'1', false, false);
    check_marker(b'2', true, true);
    check_marker(b'3', false, false);
    check_marker(b'4', false, false);
    check_marker(b'5', true, true);
    check_marker(b'6', false, false);
    check_marker(b'7', false, false);
    check_marker(b'8', true, true);
    check_marker(b'9', false, false);
    check_marker(b'A', false, false);
    check_marker(b'B', true, true);
    check_marker(b'C', false, false);
}
