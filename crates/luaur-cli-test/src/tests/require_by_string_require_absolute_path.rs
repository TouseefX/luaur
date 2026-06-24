//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/RequireByString.test.cpp:627:require_by_string_require_absolute_path`
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
//!   - calls -> method ReplWithPathFixture::runProtectedRequire (tests/RequireByString.test.cpp)
//!   - calls -> method ReplWithPathFixture::assertOutputContainsAll (tests/RequireByString.test.cpp)
//!   - calls -> method RuntimeLuauConfigTimer::start (Require/src/Navigation.cpp)
//!   - translates_to -> rust_item require_by_string_require_absolute_path

#[cfg(test)]
#[test]
fn require_by_string_require_absolute_path() {
    use crate::methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require;
    use crate::records::repl_with_path_fixture::ReplWithPathFixture;
    use alloc::string::String;

    let mut fixture = ReplWithPathFixture::new();
    let absolute_path = String::from("/an/absolute/path");
    repl_with_path_fixture_run_protected_require(&fixture, &absolute_path);
    fixture.assert_output_contains_all(&[
        String::from("false"),
        String::from("require path must start with a valid prefix: ./, ../, or @"),
    ]);
}
