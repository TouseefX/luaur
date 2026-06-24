//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3123:fragment_autocomplete_free_type_in_old_solver_shouldnt_trigger_not_null_assertion`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_free_type_in_old_solver_shouldnt_trigger_not_null_assertion() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"--!strict
local foo
local a, z = foo()

local e = foo().x

local f = foo().y

z
"#,
    );

    let dest = String::from(
        r#"--!strict
local foo
local a, z = foo()

local e = foo().x

local f = foo().y

z:a@1
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|_: &mut FragmentAutocompleteStatusResult| {}),
        None,
    );
}
