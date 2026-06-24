//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5130:autocomplete_autocomplete_string_singleton_disjoint_intersection_arg`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_autocomplete_string_singleton_disjoint_intersection_arg

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_string_singleton_disjoint_intersection_arg() {
    use crate::records::ac_fixture::AcFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;
    use luaur_common::FFlag;

    let _intersection =
        ScopedFastFlag::new(&FFlag::LuauAutocompleteStringSingletonIntersection, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        local function f(_: "foo"&"baz") end
        f("@1")
        f(@2)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac.entry_map.contains_key("foo"));
    assert!(ac.entry_map.contains_key("baz"));
    assert_eq!(ac.context, AutocompleteContext::String);

    let ac = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);
    assert!(ac.entry_map.contains_key("\"foo\""));
    assert!(ac.entry_map.contains_key("\"baz\""));
    assert_eq!(ac.context, AutocompleteContext::Expression);
}
