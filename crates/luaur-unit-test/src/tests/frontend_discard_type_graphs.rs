//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:903:frontend_discard_type_graphs`
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
//!   - type_ref -> record Frontend (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - translates_to -> rust_item frontend_discard_type_graphs

#[cfg(test)]
#[test]
fn frontend_discard_type_graphs() {
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
    let mut fe = Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
        mode,
        &mut file_resolver.base,
        &mut config_resolver.base,
        FrontendOptions::default(),
    );
    unsafe {
        fe.wire_self_pointers();
    }

    file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
        local a = {1,2,3,4,5}
    "#,
        ),
    );

    let _result = fe.check_module_name_optional_frontend_options(&String::from("Module/A"), None);

    let module = fe.module_resolver.get_module(&String::from("Module/A"));

    assert_eq!(0, module.internal_types.types.size());
    assert_eq!(0, module.internal_types.type_packs.size());
    assert_eq!(0, module.ast_types.size());
    assert_eq!(0, module.ast_resolved_types.size());
    assert_eq!(0, module.ast_resolved_type_packs.size());
}
