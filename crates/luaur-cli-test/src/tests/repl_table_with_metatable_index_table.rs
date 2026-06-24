//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/Repl.test.cpp:232:repl_table_with_metatable_index_table`
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
//!   - translates_to -> rust_item repl_table_with_metatable_index_table

#[cfg(test)]
#[test]
fn repl_table_with_metatable_index_table() {
    use crate::functions::run_code::run_code;
    use crate::methods::repl_fixture_check_completion::repl_fixture_check_completion;
    use crate::records::repl_fixture::ReplFixture;

    let mut fixture = ReplFixture::new();
    run_code(
        fixture.l as *mut _,
        &alloc::string::String::from(
            r#"
        -- Create 't' which is a table with a metatable with an __index table
        mt = {}
        mt.__index = mt

        t = {}
        setmetatable(t, mt)

        mt.mtkey1 = {x="x value", y="y value", 1, 2}
        mt.mtkey2 = 2

        t.tkey1 = {data1 = 2, data2 = "str", 3, 4}
        t.tkey2 = 4
"#,
        ),
    );

    {
        let completions = fixture.get_completion_set("t.t");
        let prefix = "t.";
        assert_eq!(2, completions.len());
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "tkey1"
        ));
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "tkey2"
        ));
    }

    {
        let completions = fixture.get_completion_set("t.tkey1.data2:re");
        let prefix = "t.tkey1.data2:";
        assert_eq!(2, completions.len());
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "rep("
        ));
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "reverse("
        ));
    }

    {
        let completions = fixture.get_completion_set("t.mtk");
        let prefix = "t.";
        assert_eq!(2, completions.len());
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "mtkey1"
        ));
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "mtkey2"
        ));
    }

    {
        let completions = fixture.get_completion_set("t.mtkey1.");
        let prefix = "t.mtkey1.";
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
