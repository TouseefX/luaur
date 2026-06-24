//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/Repl.test.cpp:323:repl_table_with_multiple_metatable_index_tables`
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
//!   - translates_to -> rust_item repl_table_with_multiple_metatable_index_tables

#[cfg(test)]
#[test]
fn repl_table_with_multiple_metatable_index_tables() {
    use crate::functions::run_code::run_code;
    use crate::methods::repl_fixture_check_completion::repl_fixture_check_completion;
    use crate::records::repl_fixture::ReplFixture;

    let mut fixture = ReplFixture::new();
    run_code(
        fixture.l as *mut _,
        &alloc::string::String::from(
            r#"
        -- Create a table with a chain of metatables
        mt2 = {}
        mt2.__index = mt2

        mt = {}
        mt.__index = mt
        setmetatable(mt, mt2)

        t = {}
        setmetatable(t, mt)

        mt2.mt2key = {x=1, y=2}
        mt.mtkey = 2
        t.tkey = 3
"#,
        ),
    );

    {
        let completions = fixture.get_completion_set("t.");
        let prefix = "t.";
        assert_eq!(4, completions.len());
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "__index"
        ));
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "tkey"
        ));
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "mtkey"
        ));
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "mt2key"
        ));
    }

    {
        let completions = fixture.get_completion_set("t.__index.");
        let prefix = "t.__index.";
        assert_eq!(3, completions.len());
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "__index"
        ));
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "mtkey"
        ));
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "mt2key"
        ));
    }

    {
        let completions = fixture.get_completion_set("t.mt2key.");
        let prefix = "t.mt2key.";
        assert_eq!(2, completions.len());
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "x"
        ));
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "y"
        ));
    }
}
