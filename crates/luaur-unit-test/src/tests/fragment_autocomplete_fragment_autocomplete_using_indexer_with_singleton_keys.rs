//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4752:fragment_autocomplete_fragment_autocomplete_using_indexer_with_singleton_keys`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_fragment_autocomplete_using_indexer_with_singleton_keys() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
        type List = "Val1" | "Val2" | "Val3"
        local Table: { [List]: boolean }
    "#,
    );

    let dest = String::from(
        r#"
        type List = "Val1" | "Val2" | "Val3"
        local Table: { [List]: boolean }
        local _ = Table.@1
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            let ac = &frag.result.as_ref().unwrap().ac_results;
            assert!(ac.entry_map.contains_key("Val1"));
            assert!(ac.entry_map.contains_key("Val2"));
            assert!(ac.entry_map.contains_key("Val3"));
        }),
        None,
    );
}
