//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3765:fragment_autocomplete_if_then_recs_else`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_if_then_recs_else() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
if x then

end
    "#,
    );

    let dest = String::from(
        r#"
if x then
e@1
end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|result: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(result.result.is_some());
            let ac = &result.result.as_ref().unwrap().ac_results;
            assert!(ac.entry_map.contains_key("else"));
            assert!(ac.entry_map.contains_key("elseif"));
        }),
        None,
    );
}
