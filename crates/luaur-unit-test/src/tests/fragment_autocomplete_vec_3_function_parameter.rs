//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2527:fragment_autocomplete_vec_3_function_parameter`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_vec_3_function_parameter() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
--!strict
local function func(abc : FakeVec)
   abc.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    // C++ `FragmentAutocompleteBuiltinsFixture::getFrontend()` virtually loads the `FakeVec`
    // class declaration into the (auto)globals on first frontend access; prime it here so the
    // shared frontend used by `autocomplete_fragment_in_both_solvers` has `FakeVec` available.
    fixture.get_frontend();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            let ac = &frag.result.as_ref().unwrap().ac_results;
            assert_eq!(2, ac.entry_map.len());
            assert!(ac.entry_map.contains_key("zero"));
            assert!(ac.entry_map.contains_key("dot"));
        }),
        None,
    );
}
