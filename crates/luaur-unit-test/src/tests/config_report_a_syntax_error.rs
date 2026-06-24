//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Config.test.cpp:49:config_report_a_syntax_error`
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
//!   - translates_to -> rust_item config_report_a_syntax_error

#[cfg(test)]
#[test]
fn config_report_a_syntax_error() {
    use luaur_config::functions::parse_config::parse_config;
    use luaur_config::records::config::Config;
    use luaur_config::records::config_options::ConfigOptions;

    let mut config = Config::default();
    let err = parse_config(
        r#"
        {"lint": {
            "UnknownGlobal": "oops"
        }}
    "#,
        &mut config,
        &ConfigOptions::default(),
    );

    assert!(err.is_some());
    assert_eq!(
        "In key UnknownGlobal: Bad setting 'oops'.  Valid options are true and false",
        err.unwrap()
    );
}
