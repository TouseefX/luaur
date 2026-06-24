//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2003:fragment_autocomplete_string_literal_with_override`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_string_literal_with_override() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_ast::records::position::Position;

    let source = String::from(
        r#"
function foo(bar: string) end
foo("a@1bc")
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '1',
        Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
            assert!(fragment.result.is_some());
            let ac_results = &fragment.result.as_ref().unwrap().ac_results;
            assert!(ac_results.entry_map.is_empty());
            assert_eq!(AutocompleteContext::String, ac_results.context);
        }),
        Some(Position { line: 2, column: 9 }),
    );
}
