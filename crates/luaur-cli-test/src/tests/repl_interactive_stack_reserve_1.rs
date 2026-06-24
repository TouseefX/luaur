//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/Repl.test.cpp:433:repl_interactive_stack_reserve_1`
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
//!   - calls -> function runCode (CLI/src/Repl.cpp)
//!   - translates_to -> rust_item repl_interactive_stack_reserve_1

#[cfg(test)]
#[test]
fn repl_interactive_stack_reserve1() {
    use crate::functions::run_code::run_code;
    use crate::records::repl_fixture::ReplFixture;

    let fixture = ReplFixture::new();

    unsafe {
        luaur_vm::functions::lua_resume::lua_resume(fixture.l as *mut _, core::ptr::null_mut(), 0);
    }

    run_code(
        fixture.l as *mut _,
        &alloc::string::String::from(
            r#"
local t = {}
"#,
        ),
    );
}
