//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3091:fragment_autocomplete_fragment_autocomplete_shouldnt_crash_on_cross_module_mutation`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_fragment_autocomplete_shouldnt_crash_on_cross_module_mutation() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"local module = {}
function module.
return module
"#,
    );

    let updated = String::from(
        r#"local module = {}
function module.f@1
return module
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &updated,
        '1',
        Box::new(|_result: &mut FragmentAutocompleteStatusResult| {}),
        None,
    );
}
