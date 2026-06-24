//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2885:fragment_autocomplete_require_tracing`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_require_tracing() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();

    fixture.base.base.base.file_resolver.source.insert(
        String::from("MainModule/A"),
        String::from(
            r#"
return { x = 0 }
    "#,
        ),
    );

    fixture.base.base.base.file_resolver.source.insert(
        String::from("MainModule"),
        String::from(
            r#"
local result = require(script.A)
local x = 1 + result.@1
    "#,
        ),
    );

    let main_module = fixture
        .base
        .base
        .base
        .file_resolver
        .source
        .get("MainModule")
        .unwrap()
        .clone();

    fixture.base.autocomplete_fragment_in_both_solvers(
        &main_module,
        &main_module,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            assert!(frag.result.as_ref().unwrap().ac_results.entry_map.len() == 1);
            assert!(frag
                .result
                .as_ref()
                .unwrap()
                .ac_results
                .entry_map
                .contains_key("x"));
        }),
        None,
    );
}
