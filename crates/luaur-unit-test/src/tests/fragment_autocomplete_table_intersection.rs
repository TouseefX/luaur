//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2369:fragment_autocomplete_table_intersection`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_table_intersection() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
        type t1 = { a1 : string, b2 : number }
        type t2 = { b2 : number, c3 : string }
        function func(abc : t1 & t2)

        end
    "#,
    );
    let updated = String::from(
        r#"
        type t1 = { a1 : string, b2 : number }
        type t2 = { b2 : number, c3 : string }
        function func(abc : t1 & t2)
            abc.@1
        end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &updated,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            let ac = &frag.result.as_ref().unwrap().ac_results;
            assert_eq!(3, ac.entry_map.len());
            assert!(ac.entry_map.contains_key("a1"));
            assert!(ac.entry_map.contains_key("b2"));
            assert!(ac.entry_map.contains_key("c3"));
            assert_eq!(AutocompleteContext::Property, ac.context);
        }),
        None,
    );
}
