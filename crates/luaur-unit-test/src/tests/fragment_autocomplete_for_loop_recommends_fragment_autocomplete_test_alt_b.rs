//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3278:fragment_autocomplete_for_loop_recommends`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_for_loop_recommends() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::fragment_autocomplete_status::FragmentAutocompleteStatus;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
local testArr: {string} = {
"a",
"b",
}

for _, v in testArr do

end
"#,
    );

    let dest = String::from(
        r#"
local testArr: {string} = {
"a",
"b",
}

for _, v in testArr do
    print(v:@1)
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|result: &mut FragmentAutocompleteStatusResult| {
            assert!(result.status != FragmentAutocompleteStatus::InternalIce);
            assert!(result.result.is_some());
            let ac = &result.result.as_ref().unwrap().ac_results;
            assert!(!ac.entry_map.is_empty());
            assert!(ac.entry_map.contains_key("upper"));
            assert!(ac.entry_map.contains_key("sub"));
        }),
        None,
    );
}
