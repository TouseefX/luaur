//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/Repl.test.cpp:163:repl_complete_global_variables`
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
//!   - translates_to -> rust_item repl_complete_global_variables

#[cfg(test)]
#[test]
fn repl_complete_global_variables() {
    use crate::functions::run_code::run_code;
    use crate::methods::repl_fixture_check_completion::repl_fixture_check_completion;
    use crate::records::repl_fixture::ReplFixture;

    let mut fixture = ReplFixture::new();
    run_code(
        fixture.l as *mut _,
        &alloc::string::String::from(
            r#"
        myvariable1 = 5
        myvariable2 = 5
"#,
        ),
    );

    {
        let completions = fixture.get_completion_set("myvar");
        let prefix = "";
        assert_eq!(2, completions.len());
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "myvariable1"
        ));
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "myvariable2"
        ));
    }

    {
        let completions = fixture.get_completion_set("math.m");
        let prefix = "math.";
        assert_eq!(4, completions.len());
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "max("
        ));
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "min("
        ));
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "modf("
        ));
        assert!(repl_fixture_check_completion(
            &fixture,
            &completions,
            prefix,
            "map("
        ));
    }
}
