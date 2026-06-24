//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/Repl.test.cpp:370:repl_table_with_deep_metatable_index_tables`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function format (Common/src/StringUtils.cpp)
//!   - type_ref -> type_alias CompletionSet (tests/Repl.test.cpp)
//!   - calls -> method ReplFixture::getCompletionSet (tests/Repl.test.cpp)
//!   - calls -> method ReplFixture::checkCompletion (tests/Repl.test.cpp)
//!   - calls -> function fail (Config/src/Config.cpp)
//!   - translates_to -> rust_item repl_table_with_deep_metatable_index_tables

#[cfg(test)]
#[test]
fn repl_table_with_deep_metatable_index_tables() {
    use crate::functions::run_code::run_code;
    use crate::methods::repl_fixture_check_completion::repl_fixture_check_completion;
    use crate::records::repl_fixture::ReplFixture;

    let mut fixture = ReplFixture::new();
    run_code(
        fixture.l as *mut _,
        &alloc::string::String::from(
            r#"
-- Creates a table with a chain of metatables of length `count`
function makeChainedTable(count)
    local result = {}
    result.__index = result
    result[string.format("entry%d", count)] = { count = count }
    if count == 0 then
        return result
    else
        return setmetatable(result, makeChainedTable(count - 1))
    end
end

t30 = makeChainedTable(30)
t60 = makeChainedTable(60)
"#,
        ),
    );

    {
        let completions = fixture.get_completion_set("t30.entry0");
        let prefix = "t30.";
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "entry0"
        ));
    }

    {
        let completions = fixture.get_completion_set("t30.entry0.co");
        let prefix = "t30.entry0.";
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "count"
        ));
    }

    {
        let completions = fixture.get_completion_set("t60.entry0");
        assert_eq!(0, completions.len());
    }

    {
        let completions = fixture.get_completion_set("t60.entry0.co");
        assert_eq!(0, completions.len());
    }
}
