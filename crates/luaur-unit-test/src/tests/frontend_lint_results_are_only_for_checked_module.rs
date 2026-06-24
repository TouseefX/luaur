//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:884:frontend_lint_results_are_only_for_checked_module`
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
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - calls -> method Fixture::lintModule (tests/Fixture.cpp)
//!   - translates_to -> rust_item frontend_lint_results_are_only_for_checked_module

#[cfg(test)]
#[test]
fn frontend_lint_results_are_only_for_checked_module() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use luaur_analysis::records::frontend_options::FrontendOptions;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
local _ = 0b10000000000000000000000000000000000000000000000000000000000000000
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("Module/B"),
        String::from(
            r#"
require(script.Parent.A)
local _ = 0x10000000000000000
    "#,
        ),
    );

    let mut opts = FrontendOptions::default();
    opts.run_lint_checks = true;

    let mut result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/B"), Some(opts.clone()));
    assert_eq!(1, result.lint_result.warnings.len());

    result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/B"), Some(opts));
    assert_eq!(1, result.lint_result.warnings.len());
}
