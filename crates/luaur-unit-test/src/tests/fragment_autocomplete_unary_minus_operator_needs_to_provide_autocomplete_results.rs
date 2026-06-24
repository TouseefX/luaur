//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4479:fragment_autocomplete_unary_minus_operator_needs_to_provide_autocomplete_results`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_unary_minus_operator_needs_to_provide_autocomplete_results() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
type Pool = { x : number }

local function foobar(p)
    local pool = p :: Pool
    if -pool
end
"#,
    );
    let dest = String::from(
        r#"
type Pool = { x : number }

local function foobar(p)
    local pool = p :: Pool
    if -pool.@1
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
        }),
        None,
    );
}
