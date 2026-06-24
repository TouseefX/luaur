//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/RequireByString.test.cpp:757:require_by_string_parse_aliases`
//! Source: `tests/RequireByString.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/RequireByString.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Config/include/Luau/Config.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file VM/include/lualib.h
//!   - includes -> source_file CLI/include/Luau/ReplRequirer.h
//!   - includes -> source_file Require/include/Luau/Require.h
//!   - includes -> source_file CLI/include/Luau/FileUtils.h
//! - incoming:
//!   - declares <- source_file tests/RequireByString.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Config (Config/include/Luau/Config.h)
//!   - type_ref -> record ConfigOptions (Config/include/Luau/Config.h)
//!   - type_ref -> record AliasOptions (Config/include/Luau/Config.h)
//!   - calls -> function parseConfig (Config/src/Config.cpp)
//!   - type_ref -> record AliasInfo (Config/include/Luau/Config.h)
//!   - translates_to -> rust_item require_by_string_parse_aliases

#[cfg(test)]
#[test]
fn require_by_string_parse_aliases() {
    use luaur_config::functions::parse_config::parse_config;
    use luaur_config::records::alias_info::AliasInfo;
    use luaur_config::records::config::Config;
    use luaur_config::records::config_options::{AliasOptions, ConfigOptions};

    fn check_contents(config: &Config) {
        assert_eq!(config.aliases.size(), 1);

        let key = "myalias".to_string();
        assert!(config.aliases.contains(&key));

        let alias_info: &AliasInfo = config.aliases.find(&key).unwrap();
        assert_eq!(alias_info.value, "/my/alias/path");
        assert_eq!(alias_info.original_case, "MyAlias");
    }

    let config_json = r#"{
    "aliases": {
        "MyAlias": "/my/alias/path",
    }
}"#;

    let mut config = Config::default();

    let alias_options = AliasOptions {
        config_location: Some("/default/location".to_string()),
        overwrite_aliases: true,
    };

    let options = ConfigOptions {
        compat: false,
        alias_options: Some(alias_options),
    };

    let error = parse_config(config_json, &mut config, &options);
    assert!(error.is_none(), "{error:?}");

    check_contents(&config);

    let copy_constructed_config = config.clone();
    check_contents(&copy_constructed_config);

    let mut copy_assigned_config = Config::default();
    copy_assigned_config.config_assign(&config);
    check_contents(&copy_assigned_config);
}
