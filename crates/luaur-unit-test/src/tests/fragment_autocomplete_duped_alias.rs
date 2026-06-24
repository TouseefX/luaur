//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2936:fragment_autocomplete_duped_alias`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_duped_alias() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_analysis::records::scope::Scope;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
type a = typeof({})

"#,
    );
    let dest = String::from(
        r#"
type a = typeof({})
type a = typeof({})@1
"#,
    );

    // Re-parsing and typechecking a type alias in the fragment that was defined in the base module will assert in ConstraintGenerator::checkAliases
    // unless we don't clone it This will let the incremental pass re-generate the type binding, and we will expect to see it in the type bindings
    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            let sc: *mut Scope = frag.result.as_ref().unwrap().fresh_scope;
            assert!(1 == unsafe { (*sc).private_type_bindings.contains_key("a") } as usize);
        }),
        None,
    );
}
