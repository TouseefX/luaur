//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Config.test.cpp:65:config_noinfer_is_still_allowed`
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
//!   - type_ref -> record ConfigOptions (Config/include/Luau/Config.h)
//!   - calls -> function parseConfig (Config/src/Config.cpp)
//!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
//!   - translates_to -> rust_item config_noinfer_is_still_allowed

#[cfg(test)]
#[test]
fn config_noinfer_is_still_allowed() {
    use luaur_ast::enums::mode::Mode;
    use luaur_config::functions::parse_config::parse_config;
    use luaur_config::records::config::Config;
    use luaur_config::records::config_options::ConfigOptions;

    let mut config = Config::default();

    let mut opts = ConfigOptions::default();
    opts.compat = true;

    let err = parse_config(r#"{ "language": {"mode": "noinfer"} }"#, &mut config, &opts);
    assert!(err.is_none());

    assert_eq!(Mode::NoCheck, config.mode);
}
