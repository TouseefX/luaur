//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/RequireByString.test.cpp:621:require_by_string_load_string_relative`
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
//!   - calls -> function runCode (CLI/src/Repl.cpp)
//!   - calls -> method ReplWithPathFixture::assertOutputContainsAll (tests/RequireByString.test.cpp)
//!   - translates_to -> rust_item require_by_string_load_string_relative

#[cfg(test)]
#[test]
fn require_by_string_load_string_relative() {
    use crate::functions::run_code::run_code;
    use crate::records::repl_with_path_fixture::ReplWithPathFixture;
    use alloc::string::String;
    use luaur_vm::records::lua_state::lua_State;

    let mut fixture = ReplWithPathFixture::new();
    let l = fixture.l as *mut lua_State;
    run_code(
        l,
        &String::from(
            "return pcall(function() return loadstring(\"require('a/relative/path')\")() end)",
        ),
    );
    fixture.assert_output_contains_all(&[
        String::from("false"),
        String::from("require is not supported in this context"),
    ]);
}
