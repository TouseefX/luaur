//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/RequireByString.test.cpp:369:require_by_string_require_init_lua`
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
//!   - calls -> method ReplWithPathFixture::getLuauDirectory (tests/RequireByString.test.cpp)
//!   - type_ref -> enum PathType (tests/RequireByString.test.cpp)
//!   - calls -> method ReplWithPathFixture::runProtectedRequire (tests/RequireByString.test.cpp)
//!   - calls -> method ReplWithPathFixture::assertOutputContainsAll (tests/RequireByString.test.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item require_by_string_require_init_lua

#[cfg(test)]
#[test]
fn require_by_string_require_init_lua() {
    use crate::enums::path_type::PathType;
    use crate::methods::repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory;
    use crate::methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require;
    use crate::records::repl_with_path_fixture::ReplWithPathFixture;
    use alloc::string::String;

    let mut fixture = ReplWithPathFixture::new();
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
        + "/tests/require/without_config/lua";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
        String::from("true"),
        String::from("result from init.lua"),
    ]);
}
