//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2608:autocomplete_suggest_table_keys`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> enum ParenthesesRecommendation (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> record Inference (Analysis/include/Luau/ConstraintGenerator.h)
//!   - translates_to -> rust_item autocomplete_suggest_table_keys

#[cfg(test)]
#[test]
fn autocomplete_suggest_table_keys() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;
    use luaur_analysis::enums::parentheses_recommendation::ParenthesesRecommendation;

    if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = AcFixture::default();

    fixture.base.check(&String::from(
        r#"
type Test = { first: number, second: number }
local t: Test = { f@1 }
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac1.entry_map.contains_key("first"));
    assert!(ac1.entry_map.contains_key("second"));
    assert_eq!(ac1.context, AutocompleteContext::Property);

    fixture.base.check(&String::from(
        r#"
type Test = { first: number } & { second: number }
local t: Test = { f@1 }
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac2.entry_map.contains_key("first"));
    assert!(ac2.entry_map.contains_key("second"));
    assert_eq!(ac2.context, AutocompleteContext::Property);

    fixture.base.check(&String::from(
        r#"
type Test = { first: number, second: number } | { second: number, third: number }
local t: Test = { s@1 }
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac3.entry_map.contains_key("second"));
    assert!(!ac3.entry_map.contains_key("first"));
    assert!(!ac3.entry_map.contains_key("third"));
    assert_eq!(ac3.context, AutocompleteContext::Property);

    fixture.base.check(&String::from(
        r#"
type Test = { first: (number) -> number, second: number }
local t: Test = { f@1 }
    "#,
    ));

    let ac4 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac4.entry_map.contains_key("first"));
    assert_eq!(
        ac4.entry_map["first"].parens,
        ParenthesesRecommendation::None
    );
    assert_eq!(ac4.context, AutocompleteContext::Property);

    fixture.base.check(&String::from(
        r#"
type Test = { first: number, second: number }
local t: Test = { f@1 = 2 }
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac5.entry_map.contains_key("first"));
    assert!(ac5.entry_map.contains_key("second"));
    assert_eq!(ac5.context, AutocompleteContext::Property);

    fixture.base.check(&String::from(
        r#"
type Test = { first: number, second: number }
local t: Test = { ["f@1"] }
    "#,
    ));

    let ac6 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac6.entry_map.contains_key("first"));
    assert!(ac6.entry_map.contains_key("second"));
    assert_eq!(ac6.context, AutocompleteContext::Property);

    fixture.base.check(&String::from(
        r#"
type Test = { first: number, second: number }
local t: Test = { "f@1" }
    "#,
    ));

    let ac7 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(!ac7.entry_map.contains_key("first"));
    assert!(!ac7.entry_map.contains_key("second"));
    assert_eq!(ac7.context, AutocompleteContext::String);

    fixture.base.check(&String::from(
        r#"
type Test = { first: number, second: number }
local t: Test = { first = 2, s@1 }
    "#,
    ));

    let ac8 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(!ac8.entry_map.contains_key("first"));
    assert!(ac8.entry_map.contains_key("second"));
    assert_eq!(ac8.context, AutocompleteContext::Property);

    fixture.base.check(&String::from(
        r#"
type Test = { first: number, second: number }
local t: Test = { first@1 }
    "#,
    ));

    let ac9 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac9.entry_map.contains_key("first"));
    assert!(ac9.entry_map.contains_key("second"));
    assert_eq!(ac9.context, AutocompleteContext::Property);

    fixture.base.check(&String::from(
        r#"
local t = {
    { first = 5, second = 10 },
    { f@1 }
}
    "#,
    ));

    let ac10 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac10.entry_map.contains_key("first"));
    assert!(ac10.entry_map.contains_key("second"));
    assert_eq!(ac10.context, AutocompleteContext::Property);

    fixture.base.check(&String::from(
        r#"
local t = {
    [2] = { first = 5, second = 10 },
    [5] = { f@1 }
}
    "#,
    ));

    let ac11 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac11.entry_map.contains_key("first"));
    assert!(ac11.entry_map.contains_key("second"));
    assert_eq!(ac11.context, AutocompleteContext::Property);
}
