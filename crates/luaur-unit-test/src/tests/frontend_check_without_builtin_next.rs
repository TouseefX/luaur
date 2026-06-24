//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1187:frontend_check_without_builtin_next`
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
//!   - type_ref -> record TestFileResolver (tests/Fixture.h)
//!   - type_ref -> record TestConfigResolver (tests/Fixture.h)
//!   - type_ref -> record Frontend (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - translates_to -> rust_item frontend_check_without_builtin_next

#[cfg(test)]
#[test]
fn frontend_check_without_builtin_next() {
    use crate::records::test_config_resolver::TestConfigResolver;
    use crate::records::test_file_resolver::TestFileResolver;
    use alloc::string::String;
    use luaur_analysis::enums::solver_mode::SolverMode;
    use luaur_analysis::records::frontend::Frontend;
    use luaur_analysis::records::frontend_options::FrontendOptions;
    use luaur_common::FFlag;

    let mut file_resolver = TestFileResolver::default();
    let mut config_resolver = TestConfigResolver::default();
    let mode = if FFlag::DebugLuauForceOldSolver.get() {
        SolverMode::Old
    } else {
        SolverMode::New
    };
    let mut frontend =
        Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
            mode,
            &mut file_resolver.base,
            &mut config_resolver.base,
            FrontendOptions::default(),
        );
    unsafe {
        frontend.wire_self_pointers();
    }

    file_resolver.source.insert(
        String::from("Module/A"),
        String::from("for k,v in 2 do end"),
    );
    file_resolver
        .source
        .insert(String::from("Module/B"), String::from("return next"));

    // We don't care about the result. That we haven't crashed is enough.
    frontend.check_module_name_optional_frontend_options(&String::from("Module/A"), None);
    frontend.check_module_name_optional_frontend_options(&String::from("Module/B"), None);
}
