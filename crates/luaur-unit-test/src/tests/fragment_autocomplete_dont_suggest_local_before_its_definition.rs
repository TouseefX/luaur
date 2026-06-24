//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2099:fragment_autocomplete_dont_suggest_local_before_its_definition`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_dont_suggest_local_before_its_definition() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
        local myLocal = 4
        function abc()
@1             local myInnerLocal = 1
@2
        end
@3    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();

    // autocomplete after abc but before myInnerLocal
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '1',
        Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(fragment.result.is_some());
            let ac = &fragment.result.as_ref().unwrap().ac_results;
            assert!(ac.entry_map.contains_key("myLocal"));
            assert!(!ac.entry_map.contains_key("myInnerLocal"));
        }),
        None,
    );
    // autocomplete after my inner local
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '2',
        Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(fragment.result.is_some());
            let ac = &fragment.result.as_ref().unwrap().ac_results;
            assert!(ac.entry_map.contains_key("myLocal"));
            assert!(ac.entry_map.contains_key("myInnerLocal"));
        }),
        None,
    );

    // autocomplete after abc, but don't include myInnerLocal(in the hidden scope)
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '3',
        Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(fragment.result.is_some());
            let ac = &fragment.result.as_ref().unwrap().ac_results;
            assert!(ac.entry_map.contains_key("myLocal"));
            assert!(!ac.entry_map.contains_key("myInnerLocal"));
        }),
        None,
    );
}
