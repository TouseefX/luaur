//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1742:fragment_autocomplete_can_autocomplete_nested_property_access`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_can_autocomplete_nested_property_access() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
local tbl = { abc = { def = 1234, egh = false } }
"#,
    );
    let updated = String::from(
        r#"
local tbl = { abc = { def = 1234, egh = false } }
tbl.abc.@1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &updated,
        '1',
        Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(fragment.result.is_some());
            LUAU_ASSERT!(!fragment.result.as_ref().unwrap().fresh_scope.is_null());

            assert_eq!(
                2,
                fragment.result.as_ref().unwrap().ac_results.entry_map.len()
            );
            assert!(fragment
                .result
                .as_ref()
                .unwrap()
                .ac_results
                .entry_map
                .contains_key("def"));
            assert!(fragment
                .result
                .as_ref()
                .unwrap()
                .ac_results
                .entry_map
                .contains_key("egh"));
            assert_eq!(
                fragment.result.as_ref().unwrap().ac_results.context,
                AutocompleteContext::Property
            );
        }),
        None,
    );
}
