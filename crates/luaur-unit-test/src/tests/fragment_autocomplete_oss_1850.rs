//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4622:fragment_autocomplete_oss_1850`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_oss_1850() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
type t = { name: "t", } | { name: "ts", person: "dog" }

local t:t
if t.name == "ts" then
end
    "#,
    );
    let dest = String::from(
        r#"
type t = { name: "t", } | { name: "ts", person: "dog" }

local t:t
if t.name == "ts" then
    t.@1
end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|result: &mut FragmentAutocompleteStatusResult| {
            let ac_results = &result.result.as_ref().unwrap().ac_results;
            assert!(!ac_results.entry_map.is_empty());
            assert!(ac_results.entry_map.contains_key("name"));
            assert!(ac_results.entry_map.contains_key("person"));
        }),
        None,
    );
}
