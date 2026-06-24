//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4254:fragment_autocomplete_for_in_should_rec`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_for_in_should_rec() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
type T = { x : {[number] : number}, y: number}
local x : T = ({} :: T)
for _,n in pairs(x.@1) do
end
"#,
    );
    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '1',
        Box::new(|result: &mut FragmentAutocompleteStatusResult| {
            assert!(!result
                .result
                .as_ref()
                .unwrap()
                .ac_results
                .entry_map
                .is_empty());
            assert!(result
                .result
                .as_ref()
                .unwrap()
                .ac_results
                .entry_map
                .contains_key("x"));
            assert!(result
                .result
                .as_ref()
                .unwrap()
                .ac_results
                .entry_map
                .contains_key("y"));
        }),
        None,
    );
}
