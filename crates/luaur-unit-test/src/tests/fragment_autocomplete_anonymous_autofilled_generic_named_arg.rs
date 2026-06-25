//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4686:fragment_autocomplete_anonymous_autofilled_generic_named_arg`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_anonymous_autofilled_generic_named_arg() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_entry_kind::AutocompleteEntryKind;
    use luaur_analysis::enums::type_correct_kind::TypeCorrectKind;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    // C++ `kGeneratedAnonymousFunctionEntryName` (AutocompleteTypes.h:92).
    const K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME: &str = "function (anonymous autofilled)";

    let source = String::from(
        "
local function foo<A>(f: (a: A) -> number, a: A)
\treturn f(a)
end
    ",
    );

    let dest = String::from(
        "
local function foo<A>(f: (a: A) -> number, a: A)
\treturn f(a)
end

foo(@1)
    ",
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            let expected_insert = "function(a): number  end";
            LUAU_ASSERT!(frag.result.is_some());
            let ac_results = &frag.result.as_ref().unwrap().ac_results;
            LUAU_ASSERT!(ac_results
                .entry_map
                .contains_key(K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME));
            let entry = &ac_results.entry_map[K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME];
            assert!(entry.kind == AutocompleteEntryKind::GeneratedFunction);
            assert!(entry.type_correct == TypeCorrectKind::Correct);
            LUAU_ASSERT!(entry.insert_text.is_some());
            assert_eq!(
                expected_insert,
                entry.insert_text.as_ref().unwrap().as_str()
            );
        }),
        None,
    );
}
