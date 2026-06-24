//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/RequireByString.test.cpp:611:require_by_string_proxy_require`
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
//!   - calls -> function luarequire_pushproxyrequire (Require/src/Require.cpp)
//!   - calls -> function requireConfigInit (CLI/src/ReplRequirer.cpp)
//!   - calls -> function createCliRequireContext (CLI/src/Repl.cpp)
//!   - calls -> macro lua_setglobal (VM/include/lua.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method ReplWithPathFixture::getLuauDirectory (tests/RequireByString.test.cpp)
//!   - type_ref -> enum PathType (tests/RequireByString.test.cpp)
//!   - calls -> method ReplWithPathFixture::runProtectedRequire (tests/RequireByString.test.cpp)
//!   - calls -> method ReplWithPathFixture::assertOutputContainsAll (tests/RequireByString.test.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item require_by_string_proxy_require

#[cfg(test)]
#[test]
fn require_by_string_proxy_require() {
    use crate::enums::path_type::PathType;
    use crate::functions::create_cli_require_context::create_cli_require_context;
    use crate::functions::require_config_init::require_config_init;
    use crate::methods::repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory;
    use crate::methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require;
    use crate::records::repl_with_path_fixture::ReplWithPathFixture;
    use alloc::string::String;
    use luaur_require::functions::luarequire_pushproxyrequire::luarequire_pushproxyrequire;
    use luaur_vm::macros::lua_setglobal::lua_setglobal;
    use luaur_vm::records::lua_state::lua_State;

    let mut fixture = ReplWithPathFixture::new();
    let l = fixture.l as *mut lua_State;

    unsafe {
        let ctx = create_cli_require_context(l);
        luarequire_pushproxyrequire(l, Some(require_config_init), ctx);
        lua_setglobal(l, c"proxyrequire".as_ptr());
    }

    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
        + "/tests/require/without_config/proxy_requirer";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
        String::from("true"),
        String::from("result from dependency"),
        String::from("required into proxy_requirer"),
    ]);
}
