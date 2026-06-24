//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4275:fragment_autocomplete_for_expr_in_should_rec_no_do`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_for_expr_in_should_rec_no_do() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
type T = { x : {[number] : number}, y: number, z: number}
local x : T = ({} :: T)
for i =
end
"#,
    );
    let dest = String::from(
        r#"
type T = { x : {[number] : number}, y: number, z : number}
local x : T = ({} :: T)
for i = x.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|result: &mut FragmentAutocompleteStatusResult| {
            let ac_results = &result.result.as_ref().unwrap().ac_results;
            assert!(!ac_results.entry_map.is_empty());
            assert!(ac_results.entry_map.contains_key("x"));
            assert!(ac_results.entry_map.contains_key("y"));
            assert!(ac_results.entry_map.contains_key("z"));
        }),
        None,
    );
}
