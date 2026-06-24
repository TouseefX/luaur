//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2520:linter_redundant_native_attribute`
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
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item linter_redundant_native_attribute

#[cfg(test)]
#[test]
fn linter_redundant_native_attribute() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
--!native

@native
local function f(a)
    @native
    local function g(b)
        return (a + b)
    end
    return g
end

f(3)(4)
"#,
        ),
        None,
    );

    assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "native attribute on a function is redundant in a native module; consider removing it",
        result.warnings[0].text.as_str()
    );
    assert_eq!(
        Location {
            begin: Position { line: 3, column: 0 },
            end: Position { line: 3, column: 7 },
        },
        result.warnings[0].location
    );
    assert_eq!(
        "native attribute on a function is redundant in a native module; consider removing it",
        result.warnings[1].text.as_str()
    );
    assert_eq!(
        Location {
            begin: Position { line: 5, column: 4 },
            end: Position {
                line: 5,
                column: 11,
            },
        },
        result.warnings[1].location
    );
}
