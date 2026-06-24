//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2644:fragment_autocomplete_bad_range_3`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_bad_range_3() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::fragment_autocomplete_status::FragmentAutocompleteStatus;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    // This test makes less sense since we don't have an updated check that
    // includes l
    // instead this will recommend nothing useful because `local t` hasn't
    // been typechecked in the fresh module
    let source = String::from(
        r#"
l
"#,
    );
    let updated = String::from(
        r#"
local t = 1
l@1
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &updated,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            assert!(frag.status == FragmentAutocompleteStatus::Success);
            LUAU_ASSERT!(frag.result.is_some());
        }),
        None,
    );
}
