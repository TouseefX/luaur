//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4436:autocomplete_anonymous_autofilled_generic_on_argument_type_pack_vararg`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_anonymous_autofilled_generic_on_argument_type_pack_vararg

#[cfg(test)]
#[test]
fn autocomplete_anonymous_autofilled_generic_on_argument_type_pack_vararg() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::autocomplete_entry_kind::AutocompleteEntryKind;
    use luaur_analysis::enums::type_correct_kind::TypeCorrectKind;
    use luaur_common::FFlag;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        local function foo(a: <T...>(...: T...) -> number)
            return a(4, 5, 6)
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
    let expected_insert = if !FFlag::DebugLuauForceOldSolver.get() {
        "function(...: number): number  end"
    } else {
        "function(...): number  end"
    };
    assert_eq!(entry.insert_text.as_deref(), Some(expected_insert));
}
