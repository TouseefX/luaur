//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2078:fragment_autocomplete_user_defined_globals`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_user_defined_globals() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from("local myLocal = 4;@1 ");

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            assert!(frag.result.is_some());
            let ac = &frag.result.as_ref().unwrap().ac_results;

            assert!(ac.entry_map.contains_key("myLocal"));
            assert!(ac.entry_map.contains_key("table"));
            assert!(ac.entry_map.contains_key("math"));
            assert_eq!(ac.context, AutocompleteContext::Statement);
        }),
        None,
    );
}
