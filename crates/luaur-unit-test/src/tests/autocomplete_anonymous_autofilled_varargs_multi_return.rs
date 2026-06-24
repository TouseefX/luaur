//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4121:autocomplete_anonymous_autofilled_varargs_multi_return`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_anonymous_autofilled_varargs_multi_return

#[cfg(test)]
#[test]
fn autocomplete_anonymous_autofilled_varargs_multi_return() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::autocomplete_entry_kind::AutocompleteEntryKind;
    use luaur_analysis::enums::type_correct_kind::TypeCorrectKind;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local function foo(a: (...number) -> (string, number))
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    let entry = ac
        .entry_map
        .get("function (anonymous autofilled)")
        .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
        entry.insert_text.as_deref(),
        Some("function(...: number): (string, number)  end")
    );
}
