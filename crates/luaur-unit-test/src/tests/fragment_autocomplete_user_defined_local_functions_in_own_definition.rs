//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2174:fragment_autocomplete_user_defined_local_functions_in_own_definition`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_user_defined_local_functions_in_own_definition() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
        local function abc()
@1
        end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    // Autocomplete inside of abc
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            assert!(frag.result.is_some());
            let ac = &frag.result.as_ref().unwrap().ac_results;
            assert!(ac.entry_map.contains_key("abc"));
            assert!(ac.entry_map.contains_key("table"));
            assert!(ac.entry_map.contains_key("math"));
        }),
        None,
    );
}
