//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:536:frontend_dont_reparse_clean_file_when_linting`
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
//!   - calls -> method LintOptions::enableWarning (Config/include/Luau/LinterConfig.h)
//!   - type_ref -> record LintWarning (Config/include/Luau/LinterConfig.h)
//!   - calls -> method Fixture::lintModule (tests/Fixture.cpp)
//!   - calls -> method Fixture::lint (tests/Fixture.cpp)
//!   - type_ref -> record Frontend (Analysis/include/Luau/Frontend.h)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - translates_to -> rust_item frontend_dont_reparse_clean_file_when_linting

#[cfg(test)]
#[test]
fn frontend_dont_reparse_clean_file_when_linting() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use luaur_analysis::records::frontend_options::FrontendOptions;
    use luaur_config::records::lint_warning::LintWarning;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("Modules/A"),
        String::from(
            r#"
        local t = {}

        for i=#t,1 do
        end

        for i=#t,1,-1 do
        end
    "#,
        ),
    );

    fixture.get_frontend();
    fixture
        .base
        .base
        .config_resolver
        .default_config
        .enabled_lint
        .enable_warning(LintWarning::Code_ForRange);

    let mut opts = FrontendOptions::default();
    opts.run_lint_checks = true;
    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(
            &String::from("Modules/A"),
            Some(opts.clone()),
        );

    fixture.base.base.file_resolver.source.insert(
        String::from("Modules/A"),
        String::from(
            r#"
        -- We have fixed the lint error, but we did not tell the Frontend that the file is changed!
        -- Therefore, we expect Frontend to reuse the results from previous lint.
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Modules/A"), Some(opts));
    assert_eq!(1, result.lint_result.warnings.len());
}
