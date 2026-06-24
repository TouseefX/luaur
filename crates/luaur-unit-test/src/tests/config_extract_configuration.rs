//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Config.test.cpp:188:config_extract_configuration`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method Fixture::lint (tests/Fixture.cpp)
//!   - type_ref -> record ConfigTable (Config/include/Luau/LuauConfig.h)
//!   - calls -> function extractConfig (Config/src/LuauConfig.cpp)
//!   - type_ref -> record InterruptCallbacks (Config/include/Luau/LuauConfig.h)
//!   - translates_to -> rust_item config_extract_configuration

#[cfg(test)]
#[test]
fn config_extract_configuration() {
    use luaur_config::functions::extract_config::extract_config;
    use luaur_config::records::config_table_key::ConfigTableKey;
    use luaur_config::records::interrupt_callbacks::InterruptCallbacks;

    let source = r#"
        local config = {}
        config.luau = {}

        config.luau.languagemode = "strict"
        config.luau.lint = {
            ["*"] = true,
            LocalUnused = false
        }
        config.luau.linterrors = true
        config.luau.typeerrors = true
        config.luau.globals = {"expect"}
        config.luau.aliases = {
            src = "./src"
        }

        return config
    "#
    .to_string();

    let mut error = String::new();
    let config_table = extract_config(&source, &InterruptCallbacks::default(), &mut error);
    assert!(config_table.is_some(), "{error}");
    let config_table = config_table.unwrap();

    assert_eq!(1, config_table.size());
    assert!(config_table.contains_str("luau"));
    let luau = config_table.find_str("luau").unwrap().get_table().unwrap();
    assert_eq!(6, luau.size());

    assert!(luau.contains_str("languagemode"));
    let language_mode = luau.find_str("languagemode").unwrap().get_string().unwrap();
    assert_eq!("strict", language_mode);

    assert!(luau.contains_str("lint"));
    let lint = luau.find_str("lint").unwrap().get_table().unwrap();
    assert_eq!(2, lint.size());
    assert!(lint.contains_str("*"));
    let all = lint.find_str("*").unwrap().get_bool().unwrap();
    assert!(*all);
    let local_unused = lint.find_str("LocalUnused").unwrap().get_bool().unwrap();
    assert!(!*local_unused);

    assert!(luau.contains_str("linterrors"));
    let lint_errors = luau.find_str("linterrors").unwrap().get_bool().unwrap();
    assert!(*lint_errors);

    assert!(luau.contains_str("typeerrors"));
    let type_errors = luau.find_str("typeerrors").unwrap().get_bool().unwrap();
    assert!(*type_errors);

    assert!(luau.contains_str("globals"));
    let globals_table = luau.find_str("globals").unwrap().get_table().unwrap();
    assert_eq!(1, globals_table.size());
    let global = globals_table
        .find(&ConfigTableKey::from(1.0))
        .unwrap()
        .get_string()
        .unwrap();
    assert_eq!("expect", global);

    assert!(luau.contains_str("aliases"));
    let aliases = luau.find_str("aliases").unwrap().get_table().unwrap();
    assert_eq!(1, aliases.size());
    assert!(aliases.contains_str("src"));
    let alias = aliases.find_str("src").unwrap().get_string().unwrap();
    assert_eq!("./src", alias);
}
