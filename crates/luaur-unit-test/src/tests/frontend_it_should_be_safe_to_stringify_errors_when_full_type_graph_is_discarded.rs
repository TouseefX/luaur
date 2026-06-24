//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:922:frontend_it_should_be_safe_to_stringify_errors_when_full_type_graph_is_discarded`
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
//!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> record TypeIds (Analysis/include/Luau/TypeIds.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item frontend_it_should_be_safe_to_stringify_errors_when_full_type_graph_is_discarded

#[cfg(test)]
#[test]
fn frontend_it_should_be_safe_to_stringify_errors_when_full_type_graph_is_discarded() {
    use crate::records::test_config_resolver::TestConfigResolver;
    use crate::records::test_file_resolver::TestFileResolver;
    use alloc::string::String;
    use luaur_analysis::enums::solver_mode::SolverMode;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
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
        --!strict
        local a: {Count: number} = {count='five'}
    "#,
        ),
    );

    let result = fe.check_module_name_optional_frontend_options(&String::from("Module/A"), None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "Table type '{ count: string }' not compatible with type '{ Count: number }' because the former is missing field 'Count'",
            to_string_type_error(&result.errors[0])
        );
    } else {
        assert_eq!(
            "Table type 'a' not compatible with type '{ Count: number }' because the former is missing field 'Count'",
            to_string_type_error(&result.errors[0])
        );
    }
}
