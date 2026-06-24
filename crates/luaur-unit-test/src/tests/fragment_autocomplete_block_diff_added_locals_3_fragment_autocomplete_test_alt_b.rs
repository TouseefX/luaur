//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3494:fragment_autocomplete_block_diff_added_locals_3`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_block_diff_added_locals_3() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::fragment_autocomplete_status::FragmentAutocompleteStatus;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"local f1 = 4
local f2 = 2 + 1"#,
    );
    let dest = String::from(
        r#"local f1 = 4
local f2 = 3
local f3 = 3
local foo = 8 + @1"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|result: &mut FragmentAutocompleteStatusResult| {
            assert_eq!(FragmentAutocompleteStatus::Success, result.status);
            LUAU_ASSERT!(result.result.is_some());
            let ac_results = &result.result.as_ref().unwrap().ac_results;
            assert!(!ac_results.entry_map.is_empty());
            assert!(ac_results.entry_map.contains_key("f1"));
            assert!(ac_results.entry_map.contains_key("f2"));
            assert!(ac_results.entry_map.contains_key("f3"));
        }),
        None,
    );
}
