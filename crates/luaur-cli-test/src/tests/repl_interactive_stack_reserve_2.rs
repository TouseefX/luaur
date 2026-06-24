//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/Repl.test.cpp:443:repl_interactive_stack_reserve_2`
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
//!   - calls -> function lua_resume (VM/src/ldo.cpp)
//!   - calls -> method ReplFixture::getCompletionSet (tests/Repl.test.cpp)
//!   - translates_to -> rust_item repl_interactive_stack_reserve_2

#[cfg(test)]
#[test]
fn repl_interactive_stack_reserve2() {
    use crate::records::repl_fixture::ReplFixture;

    let mut fixture = ReplFixture::new();

    unsafe {
        luaur_vm::functions::lua_resume::lua_resume(fixture.l as *mut _, core::ptr::null_mut(), 0);
    }

    fixture.get_completion_set("a");
}
