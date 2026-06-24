//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3151:fragment_autocomplete_interior_free_types_assertion_caused_by_free_type_inheriting_null_scope_from_table`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_interior_free_types_assertion_caused_by_free_type_inheriting_null_scope_from_table(
) {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"--!strict
local foo
local a = foo()

local e = foo().x

local f = foo().y


"#,
    );

    let dest = String::from(
        r#"--!strict
local foo
local a = foo()

local e = foo().x

local f = foo().y

z = a.P.E@1
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
