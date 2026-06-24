//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2290:fragment_autocomplete_unsealed_table_2`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_unsealed_table_2() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
        local tbl = {}
        local inner = { prop = 5 }
        tbl.inner = inner
        tbl.inner.@1
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &source,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            let ac = &frag.result.as_ref().unwrap().ac_results;
            assert_eq!(1, ac.entry_map.len());
            assert!(ac.entry_map.contains_key("prop"));
            assert_eq!(ac.context, AutocompleteContext::Property);
        }),
        None,
    );
}
