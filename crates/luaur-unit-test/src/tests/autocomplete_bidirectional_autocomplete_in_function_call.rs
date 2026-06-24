//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4705:autocomplete_bidirectional_autocomplete_in_function_call`
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
//!   - translates_to -> rust_item autocomplete_bidirectional_autocomplete_in_function_call

#[cfg(test)]
#[test]
fn autocomplete_bidirectional_autocomplete_in_function_call() {
    use crate::records::ac_fixture::AcFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        local function take(_: { choice: "left" | "right" }) end

        take({ choice = "@1" })
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(ac.entry_map.contains_key("left"), true);
    assert_eq!(ac.entry_map.contains_key("right"), true);
}
