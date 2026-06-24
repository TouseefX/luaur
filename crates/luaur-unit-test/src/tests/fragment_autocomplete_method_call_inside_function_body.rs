//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2445:fragment_autocomplete_method_call_inside_function_body`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_method_call_inside_function_body() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
        local game = { GetService=function(s) return 'hello' end }

        function a()

        end
    "#,
    );

    let updated = String::from(
        r#"
        local game = { GetService=function(s) return 'hello' end }

        function a()
            game:@1
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
            assert_ne!(0, ac.entry_map.len());

            assert!(!ac.entry_map.contains_key("math"));
            assert_eq!(ac.context, AutocompleteContext::Property);
        }),
        None,
    );
}
