//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4429:fragment_autocomplete_hot_comment_should_rec`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_hot_comment_should_rec() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(r#"--!@1"#);

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '1',
        Box::new(|result: &mut FragmentAutocompleteStatusResult| {
            let ac = &result.result.as_ref().unwrap().ac_results;
            assert!(!ac.entry_map.is_empty());
            assert!(ac.entry_map.contains_key("strict"));
            assert!(ac.entry_map.contains_key("nonstrict"));
            assert!(ac.entry_map.contains_key("nocheck"));
            assert!(ac.entry_map.contains_key("native"));
            assert!(ac.entry_map.contains_key("nolint"));
            assert!(ac.entry_map.contains_key("optimize"));
        }),
        None,
    );
}
