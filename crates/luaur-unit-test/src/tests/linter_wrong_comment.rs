//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2339:linter_wrong_comment`
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
//!   - type_ref -> record Comment (Ast/include/Luau/ParseResult.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item linter_wrong_comment

#[cfg(test)]
#[test]
fn linter_wrong_comment() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
--!strict
--!struct
--!nolintGlobal
--!nolint Global
--!nolint KnownGlobal
--!nolint UnknownGlobal
--! no more lint
--!strict here
--!native on
do end
--!nolint
"#,
        ),
        None,
    );

    assert_eq!(7, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Unknown comment directive 'struct'; did you mean 'strict'?",
        result.warnings[0].text.as_str()
    );
    assert_eq!(
        "Unknown comment directive 'nolintGlobal'",
        result.warnings[1].text.as_str()
    );
    assert_eq!(
        "nolint directive refers to unknown lint rule 'Global'",
        result.warnings[2].text.as_str()
    );
    assert_eq!(
        "nolint directive refers to unknown lint rule 'KnownGlobal'; did you mean 'UnknownGlobal'?",
        result.warnings[3].text.as_str()
    );
    assert_eq!(
        "Comment directive with the type checking mode has extra symbols at the end of the line",
        result.warnings[4].text.as_str()
    );
    assert_eq!(
        "native directive has extra symbols at the end of the line",
        result.warnings[5].text.as_str()
    );
    assert_eq!(
        "Comment directive is ignored because it is placed after the first non-comment token",
        result.warnings[6].text.as_str()
    );
}
