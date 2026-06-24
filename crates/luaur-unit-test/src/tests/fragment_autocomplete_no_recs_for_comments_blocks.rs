//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2712:fragment_autocomplete_no_recs_for_comments_blocks`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_no_recs_for_comments_blocks() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
--[[
comment 1
@1]]@2 local
-- [[ comment 2]]
--
-- sdfsdfsdf
--[[comment 3]]
--[[  @3
foo
@4bar
baz
]]
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
            LUAU_ASSERT!(frag.result.is_some());
            assert!(!frag.result.as_ref().unwrap().ac_results.entry_map.is_empty());
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
}
