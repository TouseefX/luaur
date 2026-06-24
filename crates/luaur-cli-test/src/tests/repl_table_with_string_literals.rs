//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/Repl.test.cpp:147:repl_table_with_string_literals`
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
//!   - translates_to -> rust_item repl_table_with_string_literals

#[cfg(test)]
#[test]
fn repl_table_with_string_literals() {
    use crate::functions::run_code::run_code;
    use crate::records::repl_fixture::ReplFixture;

    let mut fixture = ReplFixture::new();
    run_code(
        fixture.l as *mut _,
        &alloc::string::String::from("return {1, 'two', 3, 'four'}"),
    );

    assert_eq!("{1, \"two\", 3, \"four\"}", fixture.get_captured_output());
}
