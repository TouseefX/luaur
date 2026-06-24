//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Config.test.cpp:23:config_language_mode`
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
//!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
//!   - translates_to -> rust_item config_language_mode

#[cfg(test)]
#[test]
fn config_language_mode() {
    use luaur_ast::enums::mode::Mode;
    use luaur_config::functions::parse_config::parse_config;
    use luaur_config::records::config::Config;
    use luaur_config::records::config_options::ConfigOptions;

    let mut config = Config::default();
    let err = parse_config(
        r#"{"languageMode":"strict"}"#,
        &mut config,
        &ConfigOptions::default(),
    );
    assert!(err.is_none());

    assert_eq!(Mode::Strict, config.mode);
}
