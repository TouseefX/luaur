use crate::functions::run_code::run_code;
use crate::records::repl_with_path_fixture::ReplWithPathFixture;
use alloc::string::String;

pub fn repl_with_path_fixture_run_protected_require(
    fixture: &ReplWithPathFixture,
    path: &str,
) -> String {
    let code = format!("return pcall(function() return require(\"{}\") end)", path);
    run_code(fixture.lua_state as *mut _, &code)
}
