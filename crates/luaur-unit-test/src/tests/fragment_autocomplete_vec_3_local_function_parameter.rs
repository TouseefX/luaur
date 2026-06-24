//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2550:fragment_autocomplete_vec_3_local_function_parameter`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_vec_3_local_function_parameter() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
--!strict
local function func(abc : FakeVec)
   abc.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            assert!(frag.result.is_some());
            let ac_results = &frag.result.as_ref().unwrap().ac_results;
            assert_eq!(2, ac_results.entry_map.len());
            assert!(ac_results.entry_map.contains_key("zero"));
            assert!(ac_results.entry_map.contains_key("dot"));
        }),
        None,
    );
}
