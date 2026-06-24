//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2862:fragment_autocomplete_fragment_autocomplete_handles_parse_errors`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_fragment_autocomplete_handles_parse_errors() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;
    use luaur_common::FInt;

    let _sfi = ScopedFastInt::new(&FInt::LuauParseErrorLimit, 1);
    let source = String::from(
        r#"

"#,
    );
    let updated = String::from(
        r#"
type A = <>random non code text here  @1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &updated,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            assert!(frag
                .result
                .as_ref()
                .unwrap()
                .ac_results
                .entry_map
                .is_empty());
        }),
        None,
    );
}
