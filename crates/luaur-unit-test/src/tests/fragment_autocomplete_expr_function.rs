//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3317:fragment_autocomplete_expr_function`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_expr_function() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::fragment_autocomplete_status::FragmentAutocompleteStatus;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
local t = {}
type Input = {x : string}
function t.Do(fn : (Input) -> ())
    if t.x == "a" then
        return
    end
end

t.Do(function (f)
    f
end)
"#,
    );

    let dest = String::from(
        r#"
local t = {}
type Input = {x : string}
function t.Do(fn : (Input) -> ())
    if t.x == "a" then
        return
    end
end

t.Do(function (f)
    f.@1
end)
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|status: &mut FragmentAutocompleteStatusResult| {
            assert!(FragmentAutocompleteStatus::Success == status.status);
            LUAU_ASSERT!(status.result.is_some());
            assert!(!status
                .result
                .as_ref()
                .unwrap()
                .ac_results
                .entry_map
                .is_empty());
            assert!(status
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
