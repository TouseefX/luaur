//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1601:autocomplete_type_correct_suggestion_in_table`
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
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_type_correct_suggestion_in_table

#[cfg(test)]
#[test]
fn autocomplete_type_correct_suggestion_in_table() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;
    use luaur_analysis::enums::type_correct_kind::TypeCorrectKind;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
type Foo = { a: number, b: string }
local a = { one = 4, two = "hello" }
local b: Foo = { a = a.@1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac1.entry_map.contains_key("one"));
    assert_eq!(ac1.entry_map["one"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac1.entry_map["two"].type_correct, TypeCorrectKind::None);
    assert_eq!(ac1.context, AutocompleteContext::Property);

    fixture.base.check(&String::from(
        r#"
type Foo = { a: number, b: string }
local a = { one = 4, two = "hello" }
local b: Foo = { b = a.@1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac2.entry_map.contains_key("two"));
    assert_eq!(ac2.entry_map["two"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac2.entry_map["one"].type_correct, TypeCorrectKind::None);
    assert_eq!(ac2.context, AutocompleteContext::Property);
}
