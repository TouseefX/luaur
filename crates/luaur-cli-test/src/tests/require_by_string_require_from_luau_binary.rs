//! Node: `cxx:Test:Luau.CLI.Test:tests/RequireByString.test.cpp:730:require_by_string_require_from_luau_binary`
//! Source: `tests/RequireByString.test.cpp:730` (faithful port)
//! Graph edges:
//! - declared_by: source_file tests/RequireByString.test.cpp
//! - outgoing:
//!   - calls -> method ReplWithPathFixture::getLuauDirectory (tests/RequireByString.test.cpp)
//!   - type_ref -> enum PathType (tests/RequireByString.test.cpp)
//!   - calls -> function replMain (CLI/src/Repl.cpp)
//!   - translates_to -> rust_item require_by_string_require_from_luau_binary

#[cfg(test)]
#[test]
fn require_by_string_require_from_luau_binary() {
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

    let dir_rel = || repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative);
    let dir_abs = || repl_with_path_fixture_get_luau_directory(&fixture, PathType::Absolute);

    let paths: vec::Vec<String> = vec![
        dir_rel() + "/tests/require/without_config/dependency.luau",
        dir_abs() + "/tests/require/without_config/dependency.luau",
        dir_rel() + "/tests/require/without_config/module.luau",
        dir_abs() + "/tests/require/without_config/module.luau",
        dir_rel() + "/tests/require/without_config/nested/init.luau",
        dir_abs() + "/tests/require/without_config/nested/init.luau",
        dir_rel() + "/tests/require/config_tests/with_config/src/submodule/init.luau",
        dir_abs() + "/tests/require/config_tests/with_config/src/submodule/init.luau",
        dir_rel() + "/tests/require/config_tests/with_config_luau/src/submodule/init.luau",
        dir_abs() + "/tests/require/config_tests/with_config_luau/src/submodule/init.luau",
    ];

    for path in &paths {
        let mut path_str = CString::new(path.as_str()).unwrap();

        let mut argv: [*mut c_char; 2] = [
            executable.as_ptr() as *mut c_char,
            path_str.as_ptr() as *mut c_char,
        ];

        assert_eq!(repl_main(2, argv.as_mut_ptr()), 0, "replMain failed for {}", path);

        let _ = &mut path_str;
    }

    let _ = &mut executable;
}
