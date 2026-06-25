//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1949:fragment_autocomplete_inline_autocomplete_picks_the_right_scope_2`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_inline_autocomplete_picks_the_right_scope_2() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_analysis::records::table_type::TableType;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
type Table = { a: number, b: number }
do
    type Table = { x: string, y: string }
end
"#,
    );

    let updated = String::from(
        r#"
type Table = { a: number, b: number }
do
    type Table = { x: string, y: string }
end
local a : T@1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &updated,
        '1',
        Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
            assert!(fragment.result.is_some());
            let result = fragment.result.as_ref().unwrap();
            LUAU_ASSERT!(!result.fresh_scope.is_null());
            assert!(result.ac_results.entry_map.contains_key("Table"));
            assert!(result.ac_results.entry_map["Table"].r#type.is_some());
            let ty =
                unsafe { follow_type_id(result.ac_results.entry_map["Table"].r#type.unwrap()) };
            let tv = unsafe { get_type_id::<TableType>(ty) };
            assert!(!tv.is_null());
            assert!(unsafe { (*tv).props.contains_key("a") });
            assert!(unsafe { (*tv).props.contains_key("b") });
        }),
        None,
    );
}
