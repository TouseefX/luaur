//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1715:fragment_autocomplete_can_autocomplete_simple_property_access`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_can_autocomplete_simple_property_access() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
local tbl = { abc = 1234}
"#,
    );
    let updated = String::from(
        r#"
local tbl = { abc = 1234}
tbl. @1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &updated,
        '1',
        Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(fragment.result.is_some());
            let ac_results = &fragment.result.as_ref().unwrap().ac_results;

            assert_eq!(1, ac_results.entry_map.len());
            assert!(ac_results.entry_map.contains_key("abc"));
            assert_eq!(AutocompleteContext::Property, ac_results.context);
        }),
        None,
    );
}
