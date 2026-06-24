//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4719:fragment_autocomplete_anonymous_autofilled_generic_return_type`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_anonymous_autofilled_generic_return_type() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_entry_kind::AutocompleteEntryKind;
    use luaur_analysis::enums::type_correct_kind::TypeCorrectKind;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    // C++ `kGeneratedAnonymousFunctionEntryName` (AutocompleteTypes.h:92).
    const K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME: &str = "function (anonymous autofilled)";

    let source = String::from(
        "
local function foo<A>(f: () -> A)
\treturn f()
end
    ",
    );

    let dest = String::from(
        "
local function foo<A>(f: () -> A)
\treturn f()
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
            let expected_insert = "function()  end";
            assert!(frag.result.is_some());
            let ac = &frag.result.as_ref().unwrap().ac_results;
            assert_eq!(
                ac.entry_map
                    .contains_key(K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME) as usize,
                1
            );
            let entry = &ac.entry_map[K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME];
            assert!(entry.kind == AutocompleteEntryKind::GeneratedFunction);
            assert!(entry.type_correct == TypeCorrectKind::Correct);
            assert!(entry.insert_text.is_some());
            assert_eq!(expected_insert, entry.insert_text.as_ref().unwrap().as_str());
        }),
        None,
    );
}
