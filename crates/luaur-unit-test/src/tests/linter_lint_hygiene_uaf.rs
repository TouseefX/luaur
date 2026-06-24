//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1481:linter_lint_hygiene_uaf`
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
//!   - translates_to -> rust_item linter_lint_hygiene_uaf

#[cfg(test)]
#[test]
fn linter_lint_hygiene_uaf() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
        local Hooty = require(workspace.A)

        local  HoHooty = require(workspace.A)

        local h: Hooty.Pointy = ruire(workspace.A)

        local h: H
        local h: Hooty.Pointy = ruire(workspace.A)

        local hh: Hooty.Pointy = ruire(workspace.A)

        local h: Hooty.Pointy = ruire(workspace.A)

        linooty.Pointy = ruire(workspace.A)

        local hh: Hooty.Pointy = ruire(workspace.A)

        local h: Hooty.Pointy = ruire(workspace.A)

        linty = ruire(workspace.A)

        local h: Hooty.Pointy = ruire(workspace.A)

        local hh: Hooty.Pointy = ruire(workspace.A)

        local h: Hooty.Pointy = ruire(workspace.A)

        local h: Hooty.Pt
    "#,
        ),
        None,
    );

    assert_eq!(12, result.warnings.len(), "{:?}", result.warnings);
}
