//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4564:fragment_autocomplete_in_place_edit_of_for_loop_before_in_keyword_returns_fragment_starting_from_for`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_in_place_edit_of_for_loop_before_in_keyword_returns_fragment_starting_from_for(
) {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
local x = {}
for i, value in x do
    print(i)
end
"#,
    );

    let dest = String::from(
        r#"
local x = {}
for @1, value in x do
    print(i)
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|result: &mut FragmentAutocompleteStatusResult| {
            let ac = &result.result.as_ref().unwrap().ac_results;
            assert!(!ac.entry_map.is_empty());
        }),
        None,
    );
}
