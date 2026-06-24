//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4590:fragment_autocomplete_bidirectionally_inferred_table_member`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_bidirectionally_inferred_table_member() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
type Foo = { foo1: string, bar1: number }
type Bar = { foo2: boolean, bar2: string }
type Baz = { foo3: number, bar3: boolean }

local X: Foo & Bar & Baz = {}
"#,
    );

    let dest = String::from(
        r#"
type Foo = { foo1: string, bar1: number }
type Bar = { foo2: boolean, bar2: string }
type Baz = { foo3: number, bar3: boolean }

local X: Foo & Bar & Baz = { f@1 }

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
            assert!(ac.entry_map.contains_key("foo1"));
            assert!(ac.entry_map.contains_key("foo2"));
            assert!(ac.entry_map.contains_key("foo3"));
        }),
        None,
    );
}
