//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3581:fragment_autocomplete_str_metata_table_finished_defining`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_str_metata_table_finished_defining() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::fragment_autocomplete_status::FragmentAutocompleteStatus;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"local function foobar(): string return "" end
local foo = f"#,
    );
    let dest = String::from(
        r#"local function foobar(): string return "" end
local foo = foobar()
foo:@1"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|res: &mut FragmentAutocompleteStatusResult| {
            assert_eq!(FragmentAutocompleteStatus::Success, res.status);
            LUAU_ASSERT!(res.result.is_some());
            let ac_results = &res.result.as_ref().unwrap().ac_results;
            assert!(!ac_results.entry_map.is_empty());
            assert!(ac_results.entry_map.contains_key("len"));
            assert!(ac_results.entry_map.contains_key("gsub"));
        }),
        None,
    );
}
