//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/RequireByString.test.cpp:1196:require_by_string_require_export_alias_2`
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
//!   - translates_to -> rust_item require_by_string_require_export_alias_2

#[cfg(test)]
#[test]
fn require_by_string_require_export_alias_2() {
    use crate::enums::path_type::PathType;
    use crate::methods::repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory;
    use crate::methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require;
    use crate::records::repl_with_path_fixture::ReplWithPathFixture;
    use alloc::string::String;

    // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, {FFlag::LuauConst2, true}};
    luaur_common::FFlag::LuauExportValueSyntax.push_test_override(true);
    luaur_common::FFlag::LuauConst2.push_test_override(true);
    struct Sff;
    impl Drop for Sff {
        fn drop(&mut self) {
            luaur_common::FFlag::LuauConst2.pop_test_override();
            luaur_common::FFlag::LuauExportValueSyntax.pop_test_override();
        }
    }
    let _sff = Sff;

    let mut fixture = ReplWithPathFixture::new();
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
        + "/tests/require/without_config/export_keyword/require_export_alias2";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[String::from("true")]);
}
