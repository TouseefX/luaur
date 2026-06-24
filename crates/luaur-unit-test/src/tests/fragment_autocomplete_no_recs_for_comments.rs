//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2770:fragment_autocomplete_no_recs_for_comments`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_no_recs_for_comments() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
-- sel @1
-- retur @2
-- fo @3
--[[ sel @4]]
local @5 -- hell@6o
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            assert!(frag.result.is_none());
        }),
        None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '2',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            assert!(frag.result.is_none());
        }),
        None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '3',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            assert!(frag.result.is_none());
        }),
        None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '4',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            assert!(frag.result.is_none());
        }),
        None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '5',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            assert!(!frag.result.as_ref().unwrap().ac_results.entry_map.is_empty());
        }),
        None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '6',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            assert!(frag.result.is_none());
        }),
        None,
    );
}
