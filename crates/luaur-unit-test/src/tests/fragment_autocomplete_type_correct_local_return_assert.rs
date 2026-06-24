//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3535:fragment_autocomplete_type_correct_local_return_assert`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_type_correct_local_return_assert() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::fragment_autocomplete_status::FragmentAutocompleteStatus;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(r#""#);
    let dest = String::from(
        r#"local function target(a: number, b: string) return a + #b end
local function bar1(a: string) reutrn a .. 'x' end
local function bar2(a: number) return -a end
return target(bar@1"#,
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
            assert!(ac.entry_map.contains_key("bar1"));
            assert!(ac.entry_map.contains_key("bar2"));
        }),
        None,
    );
}
