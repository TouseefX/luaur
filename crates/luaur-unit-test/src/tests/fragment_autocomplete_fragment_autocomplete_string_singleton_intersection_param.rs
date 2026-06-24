//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4803:fragment_autocomplete_fragment_autocomplete_string_singleton_intersection_param`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_fragment_autocomplete_string_singleton_intersection_param() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::LuauAutocompleteStringSingletonIntersection, true);

    let source = String::from(
        r#"
        local function C(_: "Example"&"Example") end
    "#,
    );

    let dest = String::from(
        r#"
        local function C(_: "Example"&"Example") end
        C(@1
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            assert!(frag
                .result
                .as_ref()
                .unwrap()
                .ac_results
                .entry_map
                .contains_key("\"Example\""));
        }),
        None,
    );
}
