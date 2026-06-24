//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2573:fragment_autocomplete_function_parameter_not_recommending_out_of_scope_argument`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_function_parameter_not_recommending_out_of_scope_argument() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
--!strict
local function foo(abd: FakeVec)
end
local function bar(abc : FakeVec)
   a@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            assert!(frag
                .result
                .as_ref()
                .unwrap()
                .ac_results
                .entry_map
                .contains_key("abc"));
            assert!(!frag
                .result
                .as_ref()
                .unwrap()
                .ac_results
                .entry_map
                .contains_key("abd"));
        }),
        None,
    );
}
