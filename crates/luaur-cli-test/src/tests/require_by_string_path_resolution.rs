//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/RequireByString.test.cpp:225:require_by_string_path_resolution`
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
//!   - calls -> function format (Common/src/StringUtils.cpp)
//!   - calls -> function resolvePath (CLI/src/FileUtils.cpp)
//!   - translates_to -> rust_item require_by_string_path_resolution

#[cfg(test)]
#[test]
fn require_by_string_path_resolution() {
    use luaur_cli_lib::functions::resolve_path::resolve_path;

    #[cfg(windows)]
    let prefix = "C:/";
    #[cfg(not(windows))]
    let prefix = "/";

    let tests: Vec<(String, String, String)> = vec![
        (
            "./dep".into(),
            "./src/modules/module.luau".into(),
            "./src/modules/dep".into(),
        ),
        (
            "../dep".into(),
            "./src/modules/module.luau".into(),
            "./src/dep".into(),
        ),
        (
            "../../dep".into(),
            "./src/modules/module.luau".into(),
            "./dep".into(),
        ),
        (
            "../../".into(),
            "./src/modules/module.luau".into(),
            "./".into(),
        ),
        (
            "./dep".into(),
            "../src/modules/module.luau".into(),
            "../src/modules/dep".into(),
        ),
        (
            "../dep".into(),
            "../src/modules/module.luau".into(),
            "../src/dep".into(),
        ),
        (
            "../../dep".into(),
            "../src/modules/module.luau".into(),
            "../dep".into(),
        ),
        (
            "../../".into(),
            "../src/modules/module.luau".into(),
            "../".into(),
        ),
        (
            "./dep".into(),
            format!("{prefix}src/modules/module.luau"),
            format!("{prefix}src/modules/dep"),
        ),
        (
            "../dep".into(),
            format!("{prefix}src/modules/module.luau"),
            format!("{prefix}src/dep"),
        ),
        (
            "../../dep".into(),
            format!("{prefix}src/modules/module.luau"),
            format!("{prefix}dep"),
        ),
        (
            "../../".into(),
            format!("{prefix}src/modules/module.luau"),
            prefix.to_string(),
        ),
        (
            "../../../".into(),
            "./src/modules/module.luau".into(),
            "../".into(),
        ),
        (
            "../../../".into(),
            "../src/modules/module.luau".into(),
            "../../".into(),
        ),
        (
            "../../../".into(),
            format!("{prefix}src/modules/module.luau"),
            prefix.to_string(),
        ),
    ];

    for (input_path, input_base_file_path, expected) in tests {
        let resolved = resolve_path(&input_path, &input_base_file_path);
        assert_eq!(resolved.as_deref(), Some(expected.as_str()));
    }
}
