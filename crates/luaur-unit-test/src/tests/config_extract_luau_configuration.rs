//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Config.test.cpp:265:config_extract_luau_configuration`
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
//!   - type_ref -> record ConfigOptions (Config/include/Luau/Config.h)
//!   - type_ref -> record AliasOptions (Config/include/Luau/Config.h)
//!   - type_ref -> record Config (Config/include/Luau/Config.h)
//!   - calls -> function extractLuauConfig (Config/src/LuauConfig.cpp)
//!   - type_ref -> record InterruptCallbacks (Config/include/Luau/LuauConfig.h)
//!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
//!   - type_ref -> record LintWarning (Config/include/Luau/LinterConfig.h)
//!   - type_ref -> enum Code (Config/include/Luau/LinterConfig.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - calls -> method LintOptions::isEnabled (Config/include/Luau/LinterConfig.h)
//!   - translates_to -> rust_item config_extract_luau_configuration

#[cfg(test)]
#[test]
fn config_extract_luau_configuration() {
    use luaur_ast::enums::mode::Mode;
    use luaur_config::enums::code::Code;
    use luaur_config::functions::extract_luau_config::extract_luau_config;
    use luaur_config::records::alias_options::AliasOptions;
    use luaur_config::records::config::Config;
    use luaur_config::records::interrupt_callbacks::InterruptCallbacks;
    use luaur_config::records::lint_warning::LintWarning;

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

    let alias_options = AliasOptions {
        config_location: Some("/some/path".to_string()),
        overwrite_aliases: true,
    };

    let mut config = Config::default();
    let error = extract_luau_config(
        &source,
        &mut config,
        Some(alias_options),
        InterruptCallbacks::default(),
    );
    assert!(error.is_none(), "{error:?}");

    assert_eq!(Mode::Strict, config.mode);

    for i in 0..=LintWarning::Code__Count as i32 {
        let code: Code = unsafe { core::mem::transmute(i) };
        if code == LintWarning::Code_LocalUnused {
            assert!(!config.enabled_lint.is_enabled(code));
        } else {
            assert!(config.enabled_lint.is_enabled(code));
        }
    }

    assert_eq!(true, config.lint_errors);
    assert_eq!(true, config.type_errors);

    assert_eq!(1, config.globals.len());
    assert_eq!("expect", config.globals[0]);

    assert_eq!(1, config.aliases.size());
    let src = String::from("src");
    assert!(config.aliases.contains(&src));
    assert_eq!("./src", config.aliases.find(&src).unwrap().value);
}
