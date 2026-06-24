//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2403:fragment_autocomplete_get_suggestions_for_the_very_start_of_the_script`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_get_suggestions_for_the_very_start_of_the_script() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"@1

        function aaa() end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            let ac = &frag.result.as_ref().unwrap().ac_results;
            assert!(ac.entry_map.contains_key("table"));
            assert_eq!(ac.context, AutocompleteContext::Statement);
        }),
        None,
    );
}
