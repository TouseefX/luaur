//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Config.test.cpp:349:config_validate_return_value`
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
//!   - calls -> method SmallVector::emplace_back (Common/include/Luau/SmallVector.h)
//!   - type_ref -> record ConfigTable (Config/include/Luau/LuauConfig.h)
//!   - calls -> function extractConfig (Config/src/LuauConfig.cpp)
//!   - type_ref -> record InterruptCallbacks (Config/include/Luau/LuauConfig.h)
//!   - translates_to -> rust_item config_validate_return_value

#[cfg(test)]
#[test]
fn config_validate_return_value() {
    use luaur_config::functions::extract_config::extract_config;
    use luaur_config::records::interrupt_callbacks::InterruptCallbacks;

    let test_cases = vec![
        ("", "configuration must return exactly one value"),
        (
            "return {}, {}",
            "configuration must return exactly one value",
        ),
        ("return 'a string'", "configuration did not return a table"),
    ];

    for (source, expected_error) in test_cases {
        let source = source.to_string();
        let mut error = String::new();
        let config_table = extract_config(&source, &InterruptCallbacks::default(), &mut error);
        assert!(config_table.is_none());
        assert_eq!(expected_error, error);
    }
}
