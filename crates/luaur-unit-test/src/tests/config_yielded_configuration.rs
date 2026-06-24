//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Config.test.cpp:315:config_yielded_configuration`
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
//!   - type_ref -> record ConfigTable (Config/include/Luau/LuauConfig.h)
//!   - calls -> function extractConfig (Config/src/LuauConfig.cpp)
//!   - type_ref -> record InterruptCallbacks (Config/include/Luau/LuauConfig.h)
//!   - translates_to -> rust_item config_yielded_configuration

#[cfg(test)]
#[test]
fn config_yielded_configuration() {
    use luaur_config::functions::extract_config::extract_config;
    use luaur_config::records::interrupt_callbacks::InterruptCallbacks;

    let source = r#"
        coroutine.yield()
    "#
    .to_string();

    let mut error = String::new();
    let config_table = extract_config(&source, &InterruptCallbacks::default(), &mut error);
    assert!(config_table.is_none());
    assert_eq!("configuration execution cannot yield", error);
}
