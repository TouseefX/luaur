//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3095:autocomplete_autocomplete_string_singletons_in_literal`
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
//!   - calls -> function fail (Config/src/Config.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_autocomplete_string_singletons_in_literal

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_string_singletons_in_literal() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;

    if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        type tagged = {tag:"cat", fieldx:number} | {tag:"dog", fieldy:number}
        local x: tagged = {tag="@1"}
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("cat"));
    assert!(ac.entry_map.contains_key("dog"));
    assert_eq!(ac.context, AutocompleteContext::String);
}
