//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:103:linter_builtin_global_write`
//! Source: `tests/Linter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Linter.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Linter.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Linter.test.cpp
//! - outgoing:
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - calls -> method Fixture::lint (tests/Fixture.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item linter_builtin_global_write

#[cfg(test)]
#[test]
fn linter_builtin_global_write() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.lint(
        &String::from(
            r#"
math = {}

function assert(x)
end

assert(5)
"#,
        ),
        None,
    );

    assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Built-in global 'math' is overwritten here; consider using a local or changing the name",
        result.warnings[0].text.as_str()
    );
    assert_eq!(
        "Built-in global 'assert' is overwritten here; consider using a local or changing the name",
        result.warnings[1].text.as_str()
    );
}
