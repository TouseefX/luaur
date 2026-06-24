//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1581:fragment_autocomplete_mixed_mode_can_autocomplete_simple_property_access`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_mixed_mode_can_autocomplete_simple_property_access() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;
    use luaur_analysis::enums::solver_mode::SolverMode;
    use luaur_ast::records::position::Position;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.base.get_frontend().set_luau_solver_mode(
        if !FFlag::DebugLuauForceOldSolver.get() {
            SolverMode::New
        } else {
            SolverMode::Old
        },
    );
    let res = fixture.base.check_old_solver(&String::from(
        r#"
local tbl = { abc = 1234}
"#,
    ));

    assert_eq!(0, res.errors.len(), "{:?}", res.errors);

    let fragment = fixture.base.autocomplete_fragment(
        &String::from(
            r#"
local tbl = { abc = 1234}
tbl.
"#,
        ),
        Position { line: 2, column: 5 },
        None,
    );
    LUAU_ASSERT!(fragment.result.is_some());
    let result = fragment.result.as_ref().unwrap();
    LUAU_ASSERT!(!result.fresh_scope.is_null());

    assert_eq!(1, result.ac_results.entry_map.len());
    assert!(result.ac_results.entry_map.contains_key("abc"));
    assert_eq!(AutocompleteContext::Property, result.ac_results.context);
}
