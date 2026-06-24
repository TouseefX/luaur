//! Node: `cxx:Test:Luau.CLI.Test:tests/RequireByString.test.cpp:414:require_by_string_require_with_ambiguity_in_alias_discovery`
//! Source: `tests/RequireByString.test.cpp:414` (faithful port)
//! Graph edges:
//! - declared_by: source_file tests/RequireByString.test.cpp
//! - outgoing:
//!   - calls -> method ReplWithPathFixture::getLuauDirectory (tests/RequireByString.test.cpp)
//!   - type_ref -> enum PathType (tests/RequireByString.test.cpp)
//!   - calls -> function replMain (CLI/src/Repl.cpp)
//!   - translates_to -> rust_item require_by_string_require_with_ambiguity_in_alias_discovery

#[cfg(test)]
#[test]
fn require_by_string_require_with_ambiguity_in_alias_discovery() {
    use crate::enums::path_type::PathType;
    use crate::functions::repl_main::repl_main;
    use crate::methods::repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory;
    use crate::records::repl_with_path_fixture::ReplWithPathFixture;
    use alloc::ffi::CString;
    use alloc::string::String;
    use alloc::vec;
    use core::ffi::c_char;

    let fixture = ReplWithPathFixture::new();

    let mut executable = CString::new("luau").unwrap();

    let paths: vec::Vec<String> = vec![
        repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
            + "/tests/require/config_tests/with_config/parent_ambiguity/folder/requirer.luau",
        repl_with_path_fixture_get_luau_directory(&fixture, PathType::Absolute)
            + "/tests/require/config_tests/with_config/parent_ambiguity/folder/requirer.luau",
        repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
            + "/tests/require/config_tests/with_config_luau/parent_ambiguity/folder/requirer.luau",
        repl_with_path_fixture_get_luau_directory(&fixture, PathType::Absolute)
            + "/tests/require/config_tests/with_config_luau/parent_ambiguity/folder/requirer.luau",
    ];

    for path in &paths {
        let mut path_str = CString::new(path.as_str()).unwrap();

        let mut argv: [*mut c_char; 2] = [
            executable.as_ptr() as *mut c_char,
            path_str.as_ptr() as *mut c_char,
        ];

        assert_eq!(repl_main(2, argv.as_mut_ptr()), 0, "replMain failed for {}", path);

        // Keep the CStrings alive for the duration of the replMain call.
        let _ = &mut path_str;
    }

    let _ = &mut executable;
}
