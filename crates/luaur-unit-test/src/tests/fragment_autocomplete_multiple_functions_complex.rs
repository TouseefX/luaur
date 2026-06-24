//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1768:fragment_autocomplete_multiple_functions_complex`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_multiple_functions_complex() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let text = String::from(
        r#"@1 local function f1(a1)@2
    local l1 = 1;@3
    g1 = 1;@4
end
@5
local function f2(a2)
    local l2 = 1;@6
    g2 = 1;
end @7
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();

    fixture.base.autocomplete_fragment_in_both_solvers(
        &text,
        &text,
        '1',
        Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(fragment.result.is_some());
            let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
            assert!(!strings.contains_key("f1"));
            assert!(!strings.contains_key("a1"));
            assert!(!strings.contains_key("l1"));
            assert!(strings.contains_key("g1"));
            assert!(!strings.contains_key("f2"));
            assert!(!strings.contains_key("a2"));
            assert!(!strings.contains_key("l2"));
            assert!(strings.contains_key("g2"));
        }),
        None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
        &text,
        &text,
        '2',
        Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(fragment.result.is_some());
            let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
            assert!(strings.contains_key("f1"));
            assert!(strings.contains_key("a1"));
            assert!(!strings.contains_key("l1"));
            assert!(strings.contains_key("g1"));
            assert!(!strings.contains_key("f2"));
            assert!(!strings.contains_key("a2"));
            assert!(!strings.contains_key("l2"));
            assert!(strings.contains_key("g2"));
        }),
        None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
        &text,
        &text,
        '3',
        Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(fragment.result.is_some());
            let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
            assert!(strings.contains_key("f1"));
            assert!(strings.contains_key("a1"));
            assert!(strings.contains_key("l1"));
            assert!(strings.contains_key("g1"));
            assert!(!strings.contains_key("f2"));
            assert!(!strings.contains_key("a2"));
            assert!(!strings.contains_key("l2"));
            assert!(strings.contains_key("g2"));
        }),
        None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
        &text,
        &text,
        '4',
        Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(fragment.result.is_some());
            let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
            assert!(strings.contains_key("f1"));
            assert!(strings.contains_key("a1"));
            assert!(strings.contains_key("l1"));
            assert!(strings.contains_key("g1"));
            assert!(!strings.contains_key("f2"));
            assert!(!strings.contains_key("a2"));
            assert!(!strings.contains_key("l2"));
            assert!(strings.contains_key("g2"));
        }),
        None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
        &text,
        &text,
        '5',
        Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(fragment.result.is_some());
            let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
            assert!(strings.contains_key("f1"));
            assert!(!strings.contains_key("a1"));
            assert!(!strings.contains_key("l1"));
            assert!(strings.contains_key("g1"));
            assert!(!strings.contains_key("f2"));
            assert!(!strings.contains_key("a2"));
            assert!(!strings.contains_key("l2"));
            assert!(strings.contains_key("g2"));
        }),
        None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
        &text,
        &text,
        '6',
        Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(fragment.result.is_some());
            let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
            assert!(strings.contains_key("f1"));
            assert!(!strings.contains_key("a1"));
            assert!(!strings.contains_key("l1"));
            assert!(strings.contains_key("g1"));
            assert!(strings.contains_key("f2"));
            assert!(strings.contains_key("a2"));
            assert!(strings.contains_key("l2"));
            assert!(strings.contains_key("g2"));
        }),
        None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
        &text,
        &text,
        '7',
        Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(fragment.result.is_some());
            let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
            assert!(strings.contains_key("f1"));
            assert!(!strings.contains_key("a1"));
            assert!(!strings.contains_key("l1"));
            assert!(strings.contains_key("g1"));
            assert!(strings.contains_key("f2"));
            assert!(!strings.contains_key("a2"));
            assert!(!strings.contains_key("l2"));
            assert!(strings.contains_key("g2"));
        }),
        None,
    );
}
