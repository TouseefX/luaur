//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Config.test.cpp:32:config_disable_a_lint_rule`
//! Source: `tests/Config.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Config.test.cpp
//! - source_includes:
//!   - includes -> source_file Config/include/Luau/Config.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Config/include/Luau/LinterConfig.h
//!   - includes -> source_file Config/include/Luau/LuauConfig.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Config.test.cpp
//! - outgoing:
//!   - type_ref -> record Config (Config/include/Luau/Config.h)
//!   - calls -> function parseConfig (Config/src/Config.cpp)
//!   - calls -> method Fixture::lint (tests/Fixture.cpp)
//!   - calls -> method LintOptions::isEnabled (Config/include/Luau/LinterConfig.h)
//!   - type_ref -> record LintWarning (Config/include/Luau/LinterConfig.h)
//!   - translates_to -> rust_item config_disable_a_lint_rule

#[cfg(test)]
#[test]
fn config_disable_a_lint_rule() {
    use luaur_config::functions::parse_config::parse_config;
    use luaur_config::records::config::Config;
    use luaur_config::records::config_options::ConfigOptions;
    use luaur_config::records::lint_warning::LintWarning;

    let mut config = Config::default();
    let err = parse_config(
        r#"
        {"lint": {
            "UnknownGlobal": false,
        }}
    "#,
        &mut config,
        &ConfigOptions::default(),
    );
    assert!(err.is_none());

    assert!(!config
        .enabled_lint
        .is_enabled(LintWarning::Code_UnknownGlobal));
    assert!(config
        .enabled_lint
        .is_enabled(LintWarning::Code_DeprecatedGlobal));
}
