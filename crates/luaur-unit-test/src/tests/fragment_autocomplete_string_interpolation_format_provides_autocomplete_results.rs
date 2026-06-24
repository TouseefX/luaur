//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4199:fragment_autocomplete_string_interpolation_format_provides_autocomplete_results`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_string_interpolation_format_provides_autocomplete_results() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
type Foo = {x : number, x1 : string, x2 : boolean}
local e: Foo = {x = 1, x1 = "1", x2 = true}
local s =
"#,
    );

    let dest = String::from(
        r#"
type Foo = {x : number, x1 : string, x2 : boolean}
local e : Foo = {x = 1, x1 = "1", x2 = true}
local s = `{e.@1 }`
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
            assert!(ac.entry_map.contains_key("x1"));
            assert!(ac.entry_map.contains_key("x2"));
        }),
        None,
    );
}
