//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/Repl.test.cpp:141:repl_string_literal`
//! Source: `tests/Repl.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Repl.test.cpp
//! - source_includes:
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file VM/include/lualib.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Repl.test.cpp
//! - outgoing:
//!   - calls -> function runCode (CLI/src/Repl.cpp)
//!   - calls -> method ReplFixture::getCapturedOutput (tests/Repl.test.cpp)
//!   - translates_to -> rust_item repl_string_literal

#[cfg(test)]
#[test]
fn repl_string_literal() {
    use crate::functions::run_code::run_code;
    use crate::records::repl_fixture::ReplFixture;

    let mut fixture = ReplFixture::new();
    run_code(
        fixture.l as *mut _,
        &alloc::string::String::from("return 'str'"),
    );

    assert_eq!("\"str\"", fixture.get_captured_output());
}
