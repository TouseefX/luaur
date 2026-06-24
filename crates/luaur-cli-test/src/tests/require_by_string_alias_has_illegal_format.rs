//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/RequireByString.test.cpp:692:require_by_string_alias_has_illegal_format`
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
//!   - translates_to -> rust_item require_by_string_alias_has_illegal_format

#[cfg(test)]
#[test]
fn require_by_string_alias_has_illegal_format() {
    use crate::methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require;
    use crate::records::repl_with_path_fixture::ReplWithPathFixture;
    use alloc::string::String;

    let mut fixture = ReplWithPathFixture::new();
    let illegal_character = String::from("@@");
    repl_with_path_fixture_run_protected_require(&fixture, &illegal_character);
    fixture.assert_output_contains_all(&[
        String::from("false"),
        String::from("@@ is not a valid alias"),
    ]);
    let path_alias1 = String::from("@.");
    repl_with_path_fixture_run_protected_require(&fixture, &path_alias1);
    fixture.assert_output_contains_all(&[
        String::from("false"),
        String::from(". is not a valid alias"),
    ]);
    let path_alias2 = String::from("@..");
    repl_with_path_fixture_run_protected_require(&fixture, &path_alias2);
    fixture.assert_output_contains_all(&[
        String::from("false"),
        String::from(".. is not a valid alias"),
    ]);
    let empty_alias = String::from("@");
    repl_with_path_fixture_run_protected_require(&fixture, &empty_alias);
    fixture.assert_output_contains_all(&[
        String::from("false"),
        String::from(" is not a valid alias"),
    ]);
}
