//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2620:fragment_autocomplete_bad_range_2`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_bad_range_2() {
    use crate::functions::linear_search_for_binding::linear_search_for_binding;
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
local t = 1
"#,
    );
    let updated = String::from(
        r#"
local t = 1
t@1
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &updated,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            let opt = linear_search_for_binding(frag.result.as_ref().unwrap().fresh_scope, "t");
            LUAU_ASSERT!(opt.is_some());
            assert_eq!("number", to_string_type_id(opt.unwrap()));
        }),
        None,
    );
}
