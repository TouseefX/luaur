//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2152:autocomplete_type_correct_function_no_parenthesis`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> enum ParenthesesRecommendation (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_type_correct_function_no_parenthesis

#[cfg(test)]
#[test]
fn autocomplete_type_correct_function_no_parenthesis() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::parentheses_recommendation::ParenthesesRecommendation;
    use luaur_analysis::enums::type_correct_kind::TypeCorrectKind;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local function target(a: (number) -> number) return a(4) end
local function bar1(a: number) return -a end
local function bar2(a: string) return a .. 'x' end

return target(b@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("bar1"));
    assert_eq!(ac.entry_map["bar1"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac.entry_map["bar1"].parens, ParenthesesRecommendation::None);
    assert_eq!(ac.entry_map["bar2"].type_correct, TypeCorrectKind::None);
}
