//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/RequireByString.test.cpp:272:require_by_string_path_normalization`
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
//!   - calls -> function normalizePath (CLI/src/FileUtils.cpp)
//!   - translates_to -> rust_item require_by_string_path_normalization

#[cfg(test)]
#[test]
fn require_by_string_path_normalization() {
    use luaur_cli_lib::functions::normalize_path::normalize_path;

    #[cfg(windows)]
    let prefix = "C:/";
    #[cfg(not(windows))]
    let prefix = "/";

    let tests: Vec<(String, String)> = vec![
        ("".into(), "./".into()),
        (".".into(), "./".into()),
        ("..".into(), "../".into()),
        ("a/relative/path".into(), "./a/relative/path".into()),
        (
            "./remove/extraneous/symbols/".into(),
            "./remove/extraneous/symbols".into(),
        ),
        (
            "./remove/extraneous//symbols".into(),
            "./remove/extraneous/symbols".into(),
        ),
        (
            "./remove/extraneous/symbols/.".into(),
            "./remove/extraneous/symbols".into(),
        ),
        (
            "./remove/extraneous/./symbols".into(),
            "./remove/extraneous/symbols".into(),
        ),
        (
            "../remove/extraneous/symbols/".into(),
            "../remove/extraneous/symbols".into(),
        ),
        (
            "../remove/extraneous//symbols".into(),
            "../remove/extraneous/symbols".into(),
        ),
        (
            "../remove/extraneous/symbols/.".into(),
            "../remove/extraneous/symbols".into(),
        ),
        (
            "../remove/extraneous/./symbols".into(),
            "../remove/extraneous/symbols".into(),
        ),
        (
            format!("{prefix}remove/extraneous/symbols/"),
            format!("{prefix}remove/extraneous/symbols"),
        ),
        (
            format!("{prefix}remove/extraneous//symbols"),
            format!("{prefix}remove/extraneous/symbols"),
        ),
        (
            format!("{prefix}remove/extraneous/symbols/."),
            format!("{prefix}remove/extraneous/symbols"),
        ),
        (
            format!("{prefix}remove/extraneous/./symbols"),
            format!("{prefix}remove/extraneous/symbols"),
        ),
        ("./remove/me/..".into(), "./remove".into()),
        ("./remove/me/../".into(), "./remove".into()),
        ("../remove/me/..".into(), "../remove".into()),
        ("../remove/me/../".into(), "../remove".into()),
        (format!("{prefix}remove/me/.."), format!("{prefix}remove")),
        (format!("{prefix}remove/me/../"), format!("{prefix}remove")),
        ("./..".into(), "../".into()),
        ("./../".into(), "../".into()),
        ("../..".into(), "../../".into()),
        ("../../".into(), "../../".into()),
        (format!("{prefix}.."), prefix.to_string()),
    ];

    for (input, expected) in tests {
        assert_eq!(normalize_path(&input), expected);
    }
}
