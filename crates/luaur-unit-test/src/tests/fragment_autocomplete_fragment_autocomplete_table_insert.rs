//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4856:fragment_autocomplete_fragment_autocomplete_table_insert`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_fragment_autocomplete_table_insert() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let src = String::from(
        r#"
        local function addToTable(t: {{ foobar: number }})
            table.insert(t, {})
        end
    "#,
    );

    let dest = String::from(
        r#"
        local function addToTable(t: {{ foobar: number }})
            table.insert(t, { f@1 })
        end
    "#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &src,
        &dest,
        '1',
        Box::new(|ac: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(ac.result.is_some());
            assert!(
                (ac.result
                    .as_ref()
                    .unwrap()
                    .ac_results
                    .entry_map
                    .contains_key("foobar") as usize)
                    > 0
            );
        }),
        None,
    );
}
