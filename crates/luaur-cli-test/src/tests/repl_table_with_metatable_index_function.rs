//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/Repl.test.cpp:282:repl_table_with_metatable_index_function`
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
//!   - type_ref -> type_alias CompletionSet (tests/Repl.test.cpp)
//!   - calls -> method ReplFixture::getCompletionSet (tests/Repl.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method ReplFixture::checkCompletion (tests/Repl.test.cpp)
//!   - translates_to -> rust_item repl_table_with_metatable_index_function

#[cfg(test)]
#[test]
fn repl_table_with_metatable_index_function() {
    use crate::functions::run_code::run_code;
    use crate::methods::repl_fixture_check_completion::repl_fixture_check_completion;
    use crate::records::repl_fixture::ReplFixture;

    let mut fixture = ReplFixture::new();
    run_code(
        fixture.l as *mut _,
        &alloc::string::String::from(
            r#"
        -- Create 't' which is a table with a metatable with an __index function
        mt = {}
        mt.__index = function(table, key)
            print("mt.__index called")
            if key == "foo" then
                return "FOO"
            elseif key == "bar" then
                return "BAR"
            else
                return nil
            end
        end

        t = {}
        setmetatable(t, mt)
        t.tkey = 0
"#,
        ),
    );

    {
        let completions = fixture.get_completion_set("t.t");
        let prefix = "t.";
        assert_eq!(1, completions.len());
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "tkey"
        ));
    }

    {
        let completions = fixture.get_completion_set("t.foo");
        assert_eq!(0, completions.len());
    }

    {
        let completions = fixture.get_completion_set("t.foo:");
        assert_eq!(0, completions.len());
    }
}
