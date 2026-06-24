//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3448:autocomplete_function_in_assignment_has_parentheses_2`
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
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> enum ParenthesesRecommendation (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_function_in_assignment_has_parentheses_2

#[cfg(test)]
#[test]
fn autocomplete_function_in_assignment_has_parentheses_2() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::parentheses_recommendation::ParenthesesRecommendation;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local bar: ((number) -> number) & (number, number) -> number)
local abc = b@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("bar"));
    assert_eq!(
        ac.entry_map["bar"].parens,
        ParenthesesRecommendation::CursorInside
    );
}
