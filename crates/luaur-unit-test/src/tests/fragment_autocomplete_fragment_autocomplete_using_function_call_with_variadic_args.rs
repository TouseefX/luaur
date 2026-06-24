//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4779:fragment_autocomplete_fragment_autocomplete_using_function_call_with_variadic_args`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_fragment_autocomplete_using_function_call_with_variadic_args() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
        local function foo(...: "Val1" | "Val2") end
    "#,
    );

    let dest = String::from(
        r#"
        local function foo(...: "Val1" | "Val2") end
        foo(@1
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            assert!(
                frag.result
                    .as_ref()
                    .unwrap()
                    .ac_results
                    .entry_map
                    .contains_key("\"Val1\"") as usize
                    == 1
            );
            assert!(
                frag.result
                    .as_ref()
                    .unwrap()
                    .ac_results
                    .entry_map
                    .contains_key("\"Val2\"") as usize
                    == 1
            );
        }),
        None,
    );
}
