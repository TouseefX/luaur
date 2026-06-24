//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3626:fragment_autocomplete_diff_multiple_blocks_on_same_line`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_diff_multiple_blocks_on_same_line() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::fragment_autocomplete_status::FragmentAutocompleteStatus;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
do local function foo() end; local x = ""; end do local function bar() end"#,
    );
    let dest = String::from(
        r#"
do local function foo() end; local x = ""; end do local function bar() end local x = {a : number}; b @1end "#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|res: &mut FragmentAutocompleteStatusResult| {
            assert_eq!(FragmentAutocompleteStatus::Success, res.status);
            LUAU_ASSERT!(res.result.is_some());
            let ac = &res.result.as_ref().unwrap().ac_results;
            assert!(!ac.entry_map.is_empty());
            assert!(ac.entry_map.contains_key("bar"));
            assert!(!ac.entry_map.contains_key("foo"));
        }),
        None,
    );
}
