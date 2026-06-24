//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3361:fragment_autocomplete_differ_1`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_differ_1() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::fragment_autocomplete_status::FragmentAutocompleteStatus;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(r#""#);
    let dest = String::from(
        r#"local tbl = { foo = 1, bar = 2 };
tbl.b@1"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|status: &mut FragmentAutocompleteStatusResult| {
            assert!(FragmentAutocompleteStatus::Success == status.status);
            LUAU_ASSERT!(status.result.is_some());
            let ac = &status.result.as_ref().unwrap().ac_results;
            assert!(!ac.entry_map.is_empty());
            assert!(ac.entry_map.contains_key("foo"));
            assert!(ac.entry_map.contains_key("bar"));
        }),
        None,
    );
}
