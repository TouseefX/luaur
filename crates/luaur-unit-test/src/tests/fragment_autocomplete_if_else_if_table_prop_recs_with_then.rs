//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3827:fragment_autocomplete_if_else_if_table_prop_recs_with_then`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_if_else_if_table_prop_recs_with_then() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
type T = {xa : number, y : number}
local t : T = {xa = 3, y = 3}

if t.x then
elseif  then
end
"#,
    );

    let dest = String::from(
        r#"
type T = {xa : number, y : number}
local t : T = {xa = 3, y = 3}

if t.x then
elseif t.@1  then
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
            assert!(!ac.entry_map.is_empty());
            assert!(ac.entry_map.contains_key("xa"));
            assert!(ac.entry_map.contains_key("y"));
            assert!(!ac.entry_map.contains_key("then"));
        }),
        None,
    );
}
