//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Config.test.cpp:119:config_issue_severity`
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
//!   - translates_to -> rust_item config_issue_severity

#[cfg(test)]
#[test]
fn config_issue_severity() {
    use luaur_config::functions::parse_config::parse_config;
    use luaur_config::records::config::Config;
    use luaur_config::records::config_options::ConfigOptions;

    let mut config = Config::default();
    assert!(!config.lint_errors);
    assert!(config.type_errors);

    let err = parse_config(
        r#"
{
    "lintErrors": true,
    "typeErrors": false,
}
"#,
        &mut config,
        &ConfigOptions::default(),
    );
    assert!(err.is_none());

    assert!(config.lint_errors);
    assert!(!config.type_errors);
}
