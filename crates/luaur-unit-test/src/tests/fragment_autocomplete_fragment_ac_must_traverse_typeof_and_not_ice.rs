//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2909:fragment_autocomplete_fragment_ac_must_traverse_typeof_and_not_ice`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_fragment_ac_must_traverse_typeof_and_not_ice() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    // This test ensures that we traverse typeof expressions for defs that are being referred to in the fragment
    // In this case, we want to ensure we populate the incremental environment with the reference to `m`
    // Without this, we would ice as we will refer to the local `m` before it's declaration
    let source = String::from(
        r#"
--!strict
local m = {}
-- and here
function m:m1() end
type nt = typeof(m)

return m
"#,
    );
    let updated = String::from(
        r#"
--!strict
local m = {}
-- and here
function m:m1() end
type nt = typeof(m)
l @1
return m
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &updated,
        '1',
        Box::new(|_: &mut FragmentAutocompleteStatusResult| {}),
        None,
    );
}
