//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4227:fragment_autocomplete_string_interpolation_format_provides_results_inside_of_function_call`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_string_interpolation_format_provides_results_inside_of_function_call() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
type T = {x : number, y : number, z : number}
local e = {x = 1, y = 2, z = 3}
print(`{e.x}`)
"#,
    );

    let dest = String::from(
        r#"
type T = {x : number, y : number, z : number}
local e = {x = 1, y = 2, z = 3}
print(`{e.x} {e.@1}`)
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
            assert!(ac.entry_map.contains_key("x"));
            assert!(ac.entry_map.contains_key("y"));
            assert!(ac.entry_map.contains_key("z"));
        }),
        None,
    );
}
