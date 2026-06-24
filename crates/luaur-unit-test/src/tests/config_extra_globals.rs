//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Config.test.cpp:140:config_extra_globals`
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
//!   - translates_to -> rust_item config_extra_globals

#[cfg(test)]
#[test]
fn config_extra_globals() {
    use luaur_config::functions::parse_config::parse_config;
    use luaur_config::records::config::Config;
    use luaur_config::records::config_options::ConfigOptions;

    let mut config = Config::default();
    let err = parse_config(
        r#"
{
    "globals": ["it", "__DEV__"],
}
"#,
        &mut config,
        &ConfigOptions::default(),
    );
    assert!(err.is_none());

    assert_eq!(2, config.globals.len());
    assert_eq!("it", config.globals[0]);
    assert_eq!("__DEV__", config.globals[1]);
}
