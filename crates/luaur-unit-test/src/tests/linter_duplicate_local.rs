//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2293:linter_duplicate_local`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
//!   - translates_to -> rust_item linter_duplicate_local

#[cfg(test)]
#[test]
fn linter_duplicate_local() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
function foo(a1, a2, a3, a1)
end

local _, _, _ = ... -- ok!
local a1, a2, a1 = ... -- not ok

local moo = {}
function moo:bar(self)
end

return foo, moo, a1, a2
"#,
        ),
        None,
    );

    assert_eq!(4, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Function parameter 'a1' already defined on column 14",
        result.warnings[0].text.as_str()
    );
    assert_eq!(
        "Variable 'a1' is never used; prefix with '_' to silence",
        result.warnings[1].text.as_str()
    );
    assert_eq!(
        "Variable 'a1' already defined on column 7",
        result.warnings[2].text.as_str()
    );
    assert_eq!(
        "Function parameter 'self' already defined implicitly",
        result.warnings[3].text.as_str()
    );
}
