//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:849:frontend_test_lint_uses_correct_config`
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
//!   - calls -> method LintOptions::enableWarning (Config/include/Luau/LinterConfig.h)
//!   - type_ref -> record LintWarning (Config/include/Luau/LinterConfig.h)
//!   - calls -> method Fixture::lintModule (tests/Fixture.cpp)
//!   - calls -> method LintOptions::disableWarning (Config/include/Luau/LinterConfig.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
//!   - type_ref -> record LintOptions (Config/include/Luau/LinterConfig.h)
//!   - translates_to -> rust_item frontend_test_lint_uses_correct_config

#[cfg(test)]
#[test]
fn frontend_test_lint_uses_correct_config() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use luaur_analysis::records::frontend_options::FrontendOptions;
    use luaur_config::records::config::Config;
    use luaur_config::records::lint_options::LintOptions;
    use luaur_config::records::lint_warning::LintWarning;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
        local t = {}

        for i=#t,1 do
        end
    "#,
        ),
    );

    let mut config = Config::default();
    config
        .enabled_lint
        .enable_warning(LintWarning::Code_ForRange);
    fixture
        .base
        .base
        .config_resolver
        .config_files
        .insert(String::from("Module/A"), config);

    let mut opts = FrontendOptions::default();
    opts.run_lint_checks = true;

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/A"), Some(opts.clone()));
    assert_eq!(1, result.lint_result.warnings.len());

    fixture
        .base
        .base
        .config_resolver
        .config_files
        .get_mut(&String::from("Module/A"))
        .expect("expected config")
        .enabled_lint
        .disable_warning(LintWarning::Code_ForRange);
    fixture
        .get_frontend()
        .mark_dirty(&String::from("Module/A"), None);

    let result2 = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/A"), Some(opts.clone()));
    assert_eq!(0, result2.lint_result.warnings.len());

    let mut override_options = LintOptions::default();
    override_options.enable_warning(LintWarning::Code_ForRange);
    fixture
        .get_frontend()
        .mark_dirty(&String::from("Module/A"), None);

    opts.enabled_lint_warnings = Some(override_options);
    let result3 = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/A"), Some(opts.clone()));
    assert_eq!(1, result3.lint_result.warnings.len());

    let mut override_options = LintOptions::default();
    override_options.disable_warning(LintWarning::Code_ForRange);
    fixture
        .get_frontend()
        .mark_dirty(&String::from("Module/A"), None);

    opts.enabled_lint_warnings = Some(override_options);
    let result4 = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/A"), Some(opts));
    assert_eq!(0, result4.lint_result.warnings.len());
}
