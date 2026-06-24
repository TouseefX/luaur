//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1772:frontend_test_invalid_dependency_tracking_per_module_resolver`
//! Source: `tests/Frontend.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Frontend.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//! - incoming:
//!   - declares <- source_file tests/Frontend.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method Frontend::setLuauSolverMode (Analysis/src/Frontend.cpp)
//!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
//!   - type_ref -> record FrontendOptions (Analysis/include/Luau/Frontend.h)
//!   - calls -> method Frontend::allModuleDependenciesValid (Analysis/src/Frontend.cpp)
//!   - translates_to -> rust_item frontend_test_invalid_dependency_tracking_per_module_resolver

#[cfg(test)]
#[test]
fn frontend_test_invalid_dependency_tracking_per_module_resolver() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::enums::solver_mode::SolverMode;
    use luaur_analysis::records::frontend_options::FrontendOptions;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture
        .get_frontend()
        .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
            SolverMode::New
        } else {
            SolverMode::Old
        });

    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/A"),
        String::from("return {hello=5, world=true}"),
    );
    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/B"),
        String::from("return require(game:GetService('Gui').Modules.A)"),
    );

    let mut opts = FrontendOptions::default();
    opts.for_autocomplete = false;

    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(
            &String::from("game/Gui/Modules/B"),
            Some(opts.clone()),
        );
    assert!(fixture
        .get_frontend()
        .all_module_dependencies_valid(&String::from("game/Gui/Modules/B"), opts.for_autocomplete));
    assert!(!fixture.get_frontend().all_module_dependencies_valid(
        &String::from("game/Gui/Modules/B"),
        !opts.for_autocomplete
    ));

    opts.for_autocomplete = true;
    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(
            &String::from("game/Gui/Modules/A"),
            Some(opts.clone()),
        );

    assert!(!fixture
        .get_frontend()
        .all_module_dependencies_valid(&String::from("game/Gui/Modules/B"), opts.for_autocomplete));
    assert!(fixture.get_frontend().all_module_dependencies_valid(
        &String::from("game/Gui/Modules/B"),
        !opts.for_autocomplete
    ));
    assert!(fixture.get_frontend().all_module_dependencies_valid(
        &String::from("game/Gui/Modules/A"),
        !opts.for_autocomplete
    ));
    assert!(fixture
        .get_frontend()
        .all_module_dependencies_valid(&String::from("game/Gui/Modules/A"), opts.for_autocomplete));
}
