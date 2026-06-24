//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2316:linter_misleading_and_or`
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
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item linter_misleading_and_or

#[cfg(test)]
#[test]
fn linter_misleading_and_or() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
_ = math.random() < 0.5 and true or 42
_ = math.random() < 0.5 and false or 42 -- misleading
_ = math.random() < 0.5 and nil or 42 -- misleading
_ = math.random() < 0.5 and 0 or 42
_ = (math.random() < 0.5 and false) or 42 -- currently ignored
"#,
        ),
        None,
    );

    assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        concat!(
            "The and-or expression always evaluates to the second alternative because the first alternative is false; ",
            "consider using if-then-else expression instead"
        ),
        result.warnings[0].text.as_str()
    );
    assert_eq!(
        concat!(
            "The and-or expression always evaluates to the second alternative because the first alternative is nil; ",
            "consider using if-then-else expression instead"
        ),
        result.warnings[1].text.as_str()
    );
}
