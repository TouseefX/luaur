//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/Repl.test.cpp:420:repl_infinite_recursion`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function fail (Config/src/Config.cpp)
//!   - calls -> function runCode (CLI/src/Repl.cpp)
//!   - translates_to -> rust_item repl_infinite_recursion

#[cfg(test)]
#[test]
fn repl_infinite_recursion() {
    use crate::functions::run_code::run_code;
    use crate::records::repl_fixture::ReplFixture;

    let fixture = ReplFixture::new();
    run_code(
        fixture.l as *mut _,
        &alloc::string::String::from(
            r#"
local NewProxyOne = newproxy(true)
local MetaTableOne = getmetatable(NewProxyOne)
MetaTableOne.__index = function()
	return NewProxyOne.Game
end
print(NewProxyOne.HelloICauseACrash)
"#,
        ),
    );
}
