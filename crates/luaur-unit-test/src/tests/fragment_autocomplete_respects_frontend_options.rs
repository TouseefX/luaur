//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1435:fragment_autocomplete_respects_frontend_options`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_respects_frontend_options() {
    use crate::functions::null_callback_autocomplete_test::null_callback;
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::solver_mode::SolverMode;
    use luaur_analysis::records::fragment_context::FragmentContext;
    use luaur_analysis::records::frontend_options::FrontendOptions;
    use luaur_ast::records::position::Position;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;
    use luaur_common::FFlag;

    // NOTE: This does not pass the new solver because it is exercising behavior
    // that is only meaningful under the old solver (whether the correct
    // module resolver is used).
    //
    // C++ `DOES_NOT_PASS_NEW_SOLVER_GUARD()` =>
    // `ScopedFastFlag{FFlag::DebugLuauForceOldSolver, !FFlag::DebugLuauForceAllNewSolverTests}`.
    let _guard = ScopedFastFlag::new(
        &FFlag::DebugLuauForceOldSolver,
        !FFlag::DebugLuauForceAllNewSolverTests.get(),
    );

    let source = String::from(
        r#"
local tbl = { abc = 1234}
t
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture
        .base
        .base
        .base
        .file_resolver
        .source
        .insert(String::from("game/A"), source.clone());

    let mut opts = FrontendOptions::default();
    opts.for_autocomplete = true;

    {
        let frontend = fixture.base.base.get_frontend();
        frontend.set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
            SolverMode::New
        } else {
            SolverMode::Old
        });
        frontend.check_module_name_optional_frontend_options(
            &String::from("game/A"),
            Some(opts.clone()),
        );
        assert!(frontend
            .module_resolver_for_autocomplete
            .modules
            .get(&String::from("game/A"))
            .is_some());
        assert!(frontend
            .module_resolver
            .modules
            .get(&String::from("game/A"))
            .is_none());
    }

    let parse_result = fixture.base.parse_helper(source.clone());
    let context =
        FragmentContext::new_with_options(source.as_str(), &parse_result, Some(opts.clone()), None);

    let frontend = fixture.base.base.get_frontend();
    let frag = luaur_analysis::functions::try_fragment_autocomplete::try_fragment_autocomplete(
        frontend,
        &String::from("game/A"),
        Position { line: 2, column: 1 },
        context,
        Box::new(null_callback),
    );

    LUAU_ASSERT!(frag.result.is_some());
    let result = frag.result.as_ref().unwrap();
    assert_eq!("game/A", result.incremental_module.name);

    let frontend = fixture.base.base.get_frontend();
    assert!(frontend
        .module_resolver_for_autocomplete
        .modules
        .get(&String::from("game/A"))
        .is_some());
    assert!(frontend
        .module_resolver
        .modules
        .get(&String::from("game/A"))
        .is_none());
}
